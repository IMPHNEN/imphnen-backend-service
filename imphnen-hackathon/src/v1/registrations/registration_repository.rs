use super::{RegistrationListQueryDto, RegistrationSchema, RegistrationStatus, UserHackathonQueryDto};
use imphnen_libs::AppState;
use imphnen_utils::get_id;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

pub struct RegistrationsRepository<'a> {
    pub state: &'a AppState,
}

impl<'a> RegistrationsRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    // ============================================
    // Create Registration
    // ============================================
    pub async fn create_registration(&self, registration: RegistrationSchema) -> Result<RegistrationSchema, String> {
        let db = &self.state.surrealdb_ws;
        let created: Option<RegistrationSchema> = db
            .create("hackathon_registrations")
            .content(registration)
            .await
            .map_err(|e| format!("Failed to create registration: {}", e))?;

        created.ok_or_else(|| "Registration creation returned None".to_string())
    }

    // ============================================
    // Get Registration by ID
    // ============================================
    pub async fn query_registration_by_id(&self, id: &Thing) -> Result<Option<RegistrationSchema>, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let registration: Option<RegistrationSchema> = db
            .select(record_key)
            .await
            .map_err(|e| format!("Failed to fetch registration: {}", e))?;

        Ok(registration)
    }

    // ============================================
    // Check if User Already Registered
    // ============================================
    pub async fn check_existing_registration(
        &self,
        hackathon_id: &Thing,
        user_id: &Thing,
    ) -> Result<Option<RegistrationSchema>, String> {
        let db = &self.state.surrealdb_ws;
        let query = r#"
            SELECT * FROM hackathon_registrations
            WHERE hackathon_id = $hackathon_id 
            AND user_id = $user_id 
            AND is_deleted = false
            LIMIT 1
        "#;

        let mut result = db
            .query(query)
            .bind(("hackathon_id", hackathon_id.clone()))
            .bind(("user_id", user_id.clone()))
            .await
            .map_err(|e| format!("Failed to check existing registration: {}", e))?;

        let registration: Option<RegistrationSchema> = result
            .take(0)
            .map_err(|e| format!("Failed to parse registration: {}", e))?;

        Ok(registration)
    }

    // ============================================
    // List Registrations for Hackathon
    // ============================================
    pub async fn query_hackathon_registrations(
        &self,
        hackathon_id: &Thing,
        status_filter: Option<RegistrationStatus>,
    ) -> Result<Vec<RegistrationListQueryDto>, String> {
        let db = &self.state.surrealdb_ws;
        
        let query = if status_filter.is_some() {
            r#"
                SELECT 
                    id,
                    hackathon_id,
                    (SELECT name FROM $parent.hackathon_id)[0].name AS hackathon_name,
                    user_id,
                    (SELECT fullname FROM $parent.user_id)[0].fullname AS user_fullname,
                    (SELECT email FROM $parent.user_id)[0].email AS user_email,
                    team_id,
                    (SELECT name FROM $parent.team_id)[0].name AS team_name,
                    status,
                    role,
                    registration_date,
                    checked_in,
                    check_in_time,
                    experience_level,
                    skills
                FROM hackathon_registrations
                WHERE hackathon_id = $hackathon_id 
                AND status = $status
                AND is_deleted = false
                ORDER BY registration_date DESC
            "#
        } else {
            r#"
                SELECT 
                    id,
                    hackathon_id,
                    (SELECT name FROM $parent.hackathon_id)[0].name AS hackathon_name,
                    user_id,
                    (SELECT fullname FROM $parent.user_id)[0].fullname AS user_fullname,
                    (SELECT email FROM $parent.user_id)[0].email AS user_email,
                    team_id,
                    (SELECT name FROM $parent.team_id)[0].name AS team_name,
                    status,
                    role,
                    registration_date,
                    checked_in,
                    check_in_time,
                    experience_level,
                    skills
                FROM hackathon_registrations
                WHERE hackathon_id = $hackathon_id 
                AND is_deleted = false
                ORDER BY registration_date DESC
            "#
        };

        let hackathon_id_clone = hackathon_id.clone();
        let mut result = if let Some(status_val) = status_filter {
            db.query(query)
                .bind(("hackathon_id", hackathon_id_clone))
                .bind(("status", status_val))
                .await
        } else {
            db.query(query)
                .bind(("hackathon_id", hackathon_id_clone))
                .await
        }
        .map_err(|e| format!("Failed to query hackathon registrations: {}", e))?;

        let registrations: Vec<RegistrationListQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse registrations: {}", e))?;

        Ok(registrations)
    }

    // ============================================
    // Get User's Hackathon Registrations
    // ============================================
    pub async fn query_user_hackathons(&self, user_id: &Thing) -> Result<Vec<UserHackathonQueryDto>, String> {
        let db = &self.state.surrealdb_ws;
        let query = r#"
            SELECT 
                id AS registration_id,
                hackathon_id,
                (SELECT name FROM $parent.hackathon_id)[0].name AS hackathon_name,
                (SELECT description FROM $parent.hackathon_id)[0].description AS hackathon_description,
                (SELECT start_date FROM $parent.hackathon_id)[0].start_date AS start_date,
                (SELECT end_date FROM $parent.hackathon_id)[0].end_date AS end_date,
                status,
                role,
                registration_date,
                checked_in,
                team_id,
                (SELECT name FROM $parent.team_id)[0].name AS team_name
            FROM hackathon_registrations
            WHERE user_id = $user_id 
            AND is_deleted = false
            ORDER BY registration_date DESC
        "#;

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Failed to query user hackathons: {}", e))?;

        let hackathons: Vec<UserHackathonQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse user hackathons: {}", e))?;

        Ok(hackathons)
    }

    // ============================================
    // Get Registration Statistics
    // ============================================
    pub async fn query_registration_stats(&self, hackathon_id: &Thing) -> Result<RegistrationStatsQueryDto, String> {
        let db = &self.state.surrealdb_ws;
        let query = r#"
            LET $hackathon = (SELECT name FROM $hackathon_id)[0].name;
            LET $regs = (SELECT * FROM hackathon_registrations WHERE hackathon_id = $hackathon_id AND is_deleted = false);
            RETURN {
                hackathon_id: $hackathon_id,
                hackathon_name: $hackathon,
                total_registrations: count($regs),
                pending: count($regs[WHERE status = 'pending']),
                approved: count($regs[WHERE status = 'approved']),
                rejected: count($regs[WHERE status = 'rejected']),
                waitlisted: count($regs[WHERE status = 'waitlisted']),
                cancelled: count($regs[WHERE status = 'cancelled']),
                checked_in: count($regs[WHERE checked_in = true]),
                team_registrations: count($regs[WHERE team_id != NONE]),
                individual_registrations: count($regs[WHERE team_id = NONE])
            };
        "#;

        let hackathon_id_clone = hackathon_id.clone();
        let mut result = db
            .query(query)
            .bind(("hackathon_id", hackathon_id_clone))
            .await
            .map_err(|e| format!("Failed to query registration stats: {}", e))?;

        let stats: Option<RegistrationStatsQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse registration stats: {}", e))?;

        stats.ok_or_else(|| "Stats query returned None".to_string())
    }

    // ============================================
    // Update Registration
    // ============================================
    pub async fn update_registration(&self, id: &Thing, registration: RegistrationSchema) -> Result<RegistrationSchema, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let updated: Option<RegistrationSchema> = db
            .update(record_key)
            .content(registration)
            .await
            .map_err(|e| format!("Failed to update registration: {}", e))?;

        updated.ok_or_else(|| "Registration update returned None".to_string())
    }

    // ============================================
    // Delete Registration (soft delete)
    // ============================================
    pub async fn delete_registration(&self, id: &Thing) -> Result<(), String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let _: Option<RegistrationSchema> = db
            .delete(record_key)
            .await
            .map_err(|e| format!("Failed to delete registration: {}", e))?;

        Ok(())
    }
}

// Helper DTO for stats query
#[derive(Debug, Serialize, Deserialize)]
pub struct RegistrationStatsQueryDto {
    pub hackathon_id: String,
    pub hackathon_name: Option<String>,
    pub total_registrations: usize,
    pub pending: usize,
    pub approved: usize,
    pub rejected: usize,
    pub waitlisted: usize,
    pub cancelled: usize,
    pub checked_in: usize,
    pub team_registrations: usize,
    pub individual_registrations: usize,
}
