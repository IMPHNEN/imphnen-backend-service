use super::{SessionDetailQueryDto, SessionListQueryDto, SessionSchema};
use imphnen_libs::AppState;
use imphnen_utils::get_id;
use serde::Deserialize;
use surrealdb::sql::Thing;

pub struct SessionsRepository<'a> {
    pub state: &'a AppState,
}

impl<'a> SessionsRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    // ============================================
    // Create Session
    // ============================================
    pub async fn create_session(&self, schema: SessionSchema) -> Result<SessionSchema, String> {
        let db = &self.state.surrealdb_ws;
        let created: Option<SessionSchema> = db
            .create("sessions")
            .content(schema)
            .await
            .map_err(|e| format!("Failed to create session: {}", e))?;

        created.ok_or_else(|| "Session creation returned None".to_string())
    }

    // ============================================
    // Get Session by ID
    // ============================================
    pub async fn query_session_by_id(&self, id: &Thing) -> Result<Option<SessionSchema>, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let session: Option<SessionSchema> = db
            .select(record_key)
            .await
            .map_err(|e| format!("Failed to fetch session: {}", e))?;

        Ok(session)
    }

    // ============================================
    // Get Session Detail with User Info
    // ============================================
    pub async fn query_session_detail(&self, id: &Thing) -> Result<Option<SessionDetailQueryDto>, String> {
        let db = &self.state.surrealdb_ws;
        let query = r#"
            SELECT 
                id,
                mentor_id,
                mentee_id,
                topic,
                description,
                scheduled_at,
                duration_minutes,
                meeting_link,
                session_type,
                status,
                feedback,
                rating,
                feedback_submitted_at,
                created_at,
                updated_at,
                (SELECT fullname FROM $parent.mentor_id.user_id)[0].fullname AS mentor_fullname,
                (SELECT fullname FROM $parent.mentee_id)[0].fullname AS mentee_fullname
            FROM type::thing($table, $id)
        "#;

        let mut result = db
            .query(query)
            .bind(("table", "sessions"))
            .bind(("id", id.id.to_string()))
            .await
            .map_err(|e| format!("Failed to query session detail: {}", e))?;

        let session: Option<SessionDetailQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse session detail: {}", e))?;

        Ok(session)
    }

    // ============================================
    // List Mentor's Sessions
    // ============================================
    pub async fn query_mentor_sessions(
        &self,
        mentor_id: &Thing,
        status_filter: Option<String>,
    ) -> Result<Vec<SessionListQueryDto>, String> {
        let query = if let Some(_status) = status_filter.as_ref() {
            r#"
                SELECT 
                    id,
                    mentor_id,
                    mentee_id,
                    topic,
                    scheduled_at,
                    duration_minutes,
                    session_type,
                    status,
                    rating,
                    created_at,
                    (SELECT fullname FROM $parent.mentee_id)[0].fullname AS mentee_fullname,
                    (SELECT email FROM $parent.mentee_id)[0].email AS mentee_email
                FROM sessions
                WHERE mentor_id = $mentor_id AND status = $status
                ORDER BY scheduled_at DESC
            "#
        } else {
            r#"
                SELECT 
                    id,
                    mentor_id,
                    mentee_id,
                    topic,
                    scheduled_at,
                    duration_minutes,
                    session_type,
                    status,
                    rating,
                    created_at,
                    (SELECT fullname FROM $parent.mentee_id)[0].fullname AS mentee_fullname,
                    (SELECT email FROM $parent.mentee_id)[0].email AS mentee_email
                FROM sessions
                WHERE mentor_id = $mentor_id
                ORDER BY scheduled_at DESC
            "#
        };

        let db = &self.state.surrealdb_ws;
        let mentor_id_clone = mentor_id.clone();
        let mut result = if let Some(status_val) = status_filter {
            db.query(query)
                .bind(("mentor_id", mentor_id_clone))
                .bind(("status", status_val))
                .await
        } else {
            db.query(query)
                .bind(("mentor_id", mentor_id_clone))
                .await
        }
        .map_err(|e| format!("Failed to query mentor sessions: {}", e))?;

        let sessions: Vec<SessionListQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse mentor sessions: {}", e))?;

        Ok(sessions)
    }

    // ============================================
    // List User's Sessions (as mentee)
    // ============================================
    pub async fn query_user_sessions(
        &self,
        user_id: &Thing,
        status_filter: Option<String>,
    ) -> Result<Vec<SessionListQueryDto>, String> {
        let query = if let Some(_status) = status_filter.as_ref() {
            r#"
                SELECT 
                    id,
                    mentor_id,
                    mentee_id,
                    topic,
                    scheduled_at,
                    duration_minutes,
                    session_type,
                    status,
                    rating,
                    created_at,
                    (SELECT fullname FROM $parent.mentee_id)[0].fullname AS mentee_fullname,
                    (SELECT email FROM $parent.mentee_id)[0].email AS mentee_email
                FROM sessions
                WHERE mentee_id = $user_id AND status = $status
                ORDER BY scheduled_at DESC
            "#
        } else {
            r#"
                SELECT 
                    id,
                    mentor_id,
                    mentee_id,
                    topic,
                    scheduled_at,
                    duration_minutes,
                    session_type,
                    status,
                    rating,
                    created_at,
                    (SELECT fullname FROM $parent.mentee_id)[0].fullname AS mentee_fullname,
                    (SELECT email FROM $parent.mentee_id)[0].email AS mentee_email
                FROM sessions
                WHERE mentee_id = $user_id
                ORDER BY scheduled_at DESC
            "#
        };

        let db = &self.state.surrealdb_ws;
        let user_id_clone = user_id.clone();
        let mut result = if let Some(status_val) = status_filter {
            db.query(query)
                .bind(("user_id", user_id_clone))
                .bind(("status", status_val))
                .await
        } else {
            db.query(query)
                .bind(("user_id", user_id_clone))
                .await
        }
        .map_err(|e| format!("Failed to query user sessions: {}", e))?;

        let sessions: Vec<SessionListQueryDto> = result
            .take(0)
            .map_err(|e| format!("Failed to parse user sessions: {}", e))?;

        Ok(sessions)
    }

    // ============================================
    // Get Booked Dates for Mentor
    // ============================================
    pub async fn query_booked_dates(&self, mentor_id: &Thing) -> Result<Vec<String>, String> {
        let query = r#"
            SELECT scheduled_at FROM sessions
            WHERE mentor_id = $mentor_id 
            AND status IN ['pending', 'confirmed']
            ORDER BY scheduled_at ASC
        "#;

        let db = &self.state.surrealdb_ws;
        let mentor_id_clone = mentor_id.clone();
        let mut result = db
            .query(query)
            .bind(("mentor_id", mentor_id_clone))
            .await
            .map_err(|e| format!("Failed to query booked dates: {}", e))?;

        #[derive(Deserialize)]
        struct DateOnly {
            scheduled_at: String,
        }

        let dates: Vec<DateOnly> = result
            .take(0)
            .map_err(|e| format!("Failed to parse booked dates: {}", e))?;

        Ok(dates.into_iter().map(|d| d.scheduled_at).collect())
    }

    // ============================================
    // Update Session
    // ============================================
    pub async fn update_session(&self, id: &Thing, schema: SessionSchema) -> Result<SessionSchema, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let updated: Option<SessionSchema> = db
            .update(record_key)
            .content(schema)
            .await
            .map_err(|e| format!("Failed to update session: {}", e))?;

        updated.ok_or_else(|| "Session update returned None".to_string())
    }

    // ============================================
    // Count Mentor Sessions
    // ============================================
    pub async fn count_mentor_sessions(
        &self,
        mentor_id: &Thing,
        status_filter: Option<String>,
    ) -> Result<usize, String> {
        let query = if status_filter.is_some() {
            "SELECT count() FROM sessions WHERE mentor_id = $mentor_id AND status = $status GROUP ALL"
        } else {
            "SELECT count() FROM sessions WHERE mentor_id = $mentor_id GROUP ALL"
        };

        let db = &self.state.surrealdb_ws;
        let mentor_id_clone = mentor_id.clone();
        let mut result = if let Some(status_val) = status_filter {
            db.query(query)
                .bind(("mentor_id", mentor_id_clone))
                .bind(("status", status_val))
                .await
        } else {
            db.query(query)
                .bind(("mentor_id", mentor_id_clone))
                .await
        }
        .map_err(|e| format!("Failed to count mentor sessions: {}", e))?;

        #[derive(serde::Deserialize)]
        struct CountResult {
            count: usize,
        }

        let count_result: Option<CountResult> = result
            .take(0)
            .map_err(|e| format!("Failed to parse count: {}", e))?;

        Ok(count_result.map(|r| r.count).unwrap_or(0))
    }

    // ============================================
    // Count User Sessions
    // ============================================
    pub async fn count_user_sessions(
        &self,
        user_id: &Thing,
        status_filter: Option<String>,
    ) -> Result<usize, String> {
        let query = if status_filter.is_some() {
            "SELECT count() FROM sessions WHERE mentee_id = $user_id AND status = $status GROUP ALL"
        } else {
            "SELECT count() FROM sessions WHERE mentee_id = $user_id GROUP ALL"
        };

        let db = &self.state.surrealdb_ws;
        let user_id_clone = user_id.clone();
        let mut result = if let Some(status_val) = status_filter {
            db.query(query)
                .bind(("user_id", user_id_clone))
                .bind(("status", status_val))
                .await
        } else {
            db.query(query)
                .bind(("user_id", user_id_clone))
                .await
        }
        .map_err(|e| format!("Failed to count user sessions: {}", e))?;

        #[derive(serde::Deserialize)]
        struct CountResult {
            count: usize,
        }

        let count_result: Option<CountResult> = result
            .take(0)
            .map_err(|e| format!("Failed to parse count: {}", e))?;

        Ok(count_result.map(|r| r.count).unwrap_or(0))
    }

    // ============================================
    // Delete Session (soft delete)
    // ============================================
    // Delete Session (soft delete)
    // ============================================
    pub async fn delete_session(&self, id: &Thing) -> Result<(), String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(id).map_err(|e| e.to_string())?;
        let _: Option<SessionSchema> = db
            .delete(record_key)
            .await
            .map_err(|e| format!("Failed to delete session: {}", e))?;

        Ok(())
    }
}

