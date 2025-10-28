use super::{ParticipantRole, RegistrationListQueryDto, RegistrationSchema, RegistrationStatus, UserHackathonQueryDto};
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
        
        // Use FETCH to retrieve related data in a single query
        let query = if status_filter.is_some() {
            r#"
                SELECT 
                    string::join(':', id.tb, id.id) AS id,
                    string::join(':', hackathon_id.tb, hackathon_id.id) AS hackathon_id,
                    hackathon_id.name AS hackathon_name,
                    string::join(':', user_id.tb, user_id.id) AS user_id,
                    user_id.fullname AS user_fullname,
                    user_id.email AS user_email,
                    (IF team_id != NONE THEN string::join(':', team_id.tb, team_id.id) ELSE NONE END) AS team_id,
                    (IF team_id != NONE THEN team_id.name ELSE NONE END) AS team_name,
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
                FETCH hackathon_id, user_id, team_id
                ORDER BY registration_date DESC
            "#
        } else {
            r#"
                SELECT 
                    string::join(':', id.tb, id.id) AS id,
                    string::join(':', hackathon_id.tb, hackathon_id.id) AS hackathon_id,
                    hackathon_id.name AS hackathon_name,
                    string::join(':', user_id.tb, user_id.id) AS user_id,
                    user_id.fullname AS user_fullname,
                    user_id.email AS user_email,
                    (IF team_id != NONE THEN string::join(':', team_id.tb, team_id.id) ELSE NONE END) AS team_id,
                    (IF team_id != NONE THEN team_id.name ELSE NONE END) AS team_name,
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
                FETCH hackathon_id, user_id, team_id
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

        // Use intermediate struct for parsing with all fields including related data
        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleReg {
            id: String,
            hackathon_id: String,
            hackathon_name: Option<String>,
            user_id: String,
            user_fullname: Option<String>,
            user_email: Option<String>,
            team_id: Option<String>,
            team_name: Option<String>,
            status: RegistrationStatus,
            role: ParticipantRole,
            registration_date: String,
            checked_in: bool,
            check_in_time: Option<String>,
            experience_level: Option<String>,
            skills: Option<Vec<String>>,
        }

        let simple: Vec<SimpleReg> = result
            .take(0)
            .map_err(|e| format!("Failed to parse registrations: {}", e))?;

        // Convert to full DTO with all fetched data
        let registrations = simple
            .into_iter()
            .map(|r| RegistrationListQueryDto {
                id: r.id,
                hackathon_id: r.hackathon_id,
                hackathon_name: r.hackathon_name,
                user_id: r.user_id,
                user_fullname: r.user_fullname,
                user_email: r.user_email,
                team_id: r.team_id,
                team_name: r.team_name,
                status: r.status,
                role: r.role,
                registration_date: r.registration_date,
                checked_in: r.checked_in,
                check_in_time: r.check_in_time,
                experience_level: r.experience_level,
                skills: r.skills,
            })
            .collect();

        Ok(registrations)
    }

    // ============================================
    // Get User's Hackathon Registrations
    // ============================================
    pub async fn query_user_hackathons(&self, user_id: &Thing) -> Result<Vec<UserHackathonQueryDto>, String> {
        let db = &self.state.surrealdb_ws;
        // Use FETCH to retrieve related hackathon and team data
        let query = r#"
            SELECT 
                string::join(':', id.tb, id.id) AS registration_id,
                string::join(':', hackathon_id.tb, hackathon_id.id) AS hackathon_id,
                hackathon_id.name AS hackathon_name,
                hackathon_id.description AS hackathon_description,
                hackathon_id.start_date AS start_date,
                hackathon_id.end_date AS end_date,
                status,
                role,
                registration_date,
                checked_in,
                (IF team_id != NONE THEN string::join(':', team_id.tb, team_id.id) ELSE NONE END) AS team_id,
                (IF team_id != NONE THEN team_id.name ELSE NONE END) AS team_name
            FROM hackathon_registrations
            WHERE user_id = $user_id 
            AND is_deleted = false
            FETCH hackathon_id, team_id
            ORDER BY registration_date DESC
        "#;

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Failed to query user hackathons: {}", e))?;

        #[derive(Debug, Serialize, Deserialize)]
        struct SimpleUserHackathon {
            registration_id: String,
            hackathon_id: String,
            hackathon_name: Option<String>,
            hackathon_description: Option<String>,
            start_date: Option<String>,
            end_date: Option<String>,
            status: RegistrationStatus,
            role: ParticipantRole,
            registration_date: String,
            checked_in: bool,
            team_id: Option<String>,
            team_name: Option<String>,
        }

        let simple: Vec<SimpleUserHackathon> = result
            .take(0)
            .map_err(|e| format!("Failed to parse user hackathons: {}", e))?;

        // Convert to full DTO with all fetched data
        let hackathons = simple
            .into_iter()
            .map(|h| UserHackathonQueryDto {
                registration_id: h.registration_id,
                hackathon_id: h.hackathon_id,
                hackathon_name: h.hackathon_name,
                hackathon_description: h.hackathon_description,
                start_date: h.start_date,
                end_date: h.end_date,
                status: h.status,
                role: h.role,
                registration_date: h.registration_date,
                checked_in: h.checked_in,
                team_id: h.team_id,
                team_name: h.team_name,
            })
            .collect();

        Ok(hackathons)
    }

    // ============================================
    // Get Registration Statistics
    // ============================================
    pub async fn query_registration_stats(&self, hackathon_id: &Thing) -> Result<RegistrationStatsQueryDto, String> {
        let db = &self.state.surrealdb_ws;
        
        // Get hackathon name first
        let hackathon_query = r#"
            SELECT name FROM hackathons WHERE id = $hackathon_id LIMIT 1
        "#;
        
        let hackathon_id_clone = hackathon_id.clone();
        let mut hackathon_result = db
            .query(hackathon_query)
            .bind(("hackathon_id", hackathon_id_clone.clone()))
            .await
            .map_err(|e| format!("Failed to fetch hackathon name: {}", e))?;
        
        #[derive(Debug, Serialize, Deserialize)]
        struct HackathonName {
            name: String,
        }
        
        let hackathon_names: Vec<HackathonName> = hackathon_result
            .take(0)
            .map_err(|e| format!("Failed to parse hackathon name: {}", e))?;
        
        let hackathon_name = hackathon_names.first().map(|h| h.name.clone());
        
        // Get all registrations
        let query = r#"
            SELECT * FROM hackathon_registrations 
            WHERE hackathon_id = $hackathon_id 
            AND is_deleted = false
        "#;

        let mut result = db
            .query(query)
            .bind(("hackathon_id", hackathon_id_clone))
            .await
            .map_err(|e| format!("Failed to query registrations for stats: {}", e))?;

        #[derive(Debug, Serialize, Deserialize)]
        struct RegForStats {
            status: RegistrationStatus,
            checked_in: bool,
            team_id: Option<String>,
        }

        let regs: Vec<RegForStats> = result
            .take(0)
            .map_err(|e| format!("Failed to parse registrations for stats: {}", e))?;

        // Calculate stats manually
        let total = regs.len();
        let pending = regs.iter().filter(|r| matches!(r.status, RegistrationStatus::Pending)).count();
        let approved = regs.iter().filter(|r| matches!(r.status, RegistrationStatus::Approved)).count();
        let rejected = regs.iter().filter(|r| matches!(r.status, RegistrationStatus::Rejected)).count();
        let waitlisted = regs.iter().filter(|r| matches!(r.status, RegistrationStatus::Waitlisted)).count();
        let cancelled = regs.iter().filter(|r| matches!(r.status, RegistrationStatus::Cancelled)).count();
        let checked_in = regs.iter().filter(|r| r.checked_in).count();
        let team_registrations = regs.iter().filter(|r| r.team_id.is_some()).count();
        let individual_registrations = regs.iter().filter(|r| r.team_id.is_none()).count();

        let hackathon_id_str = format!("{}", hackathon_id);
        
        Ok(RegistrationStatsQueryDto {
            hackathon_id: hackathon_id_str,
            hackathon_name,
            total_registrations: total,
            pending,
            approved,
            rejected,
            waitlisted,
            cancelled,
            checked_in,
            team_registrations,
            individual_registrations,
        })
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
