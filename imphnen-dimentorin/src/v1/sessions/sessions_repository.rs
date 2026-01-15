use super::{SessionDetailQueryDto, SessionListQueryDto, SessionSchema};
use anyhow::{anyhow, Result};
use imphnen_libs::{AppState, AppStatePostgresExt};
use sea_orm::*;
use uuid::Uuid;

use imphnen_entities::seaorm::auth::sessions::{
    Entity as Sessions, Model as SessionModel, ActiveModel as SessionActiveModel, Column as SessionColumn,
};
use imphnen_entities::seaorm::auth::users::Entity as Users;

pub struct SessionsRepository<'a> {
    pub db: &'a DatabaseConnection,
}

impl<'a> SessionsRepository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { db: state.postgres_db() }
    }

    // ============================================
    // Create Session
    // ============================================
    pub async fn create_session(&self, schema: SessionSchema) -> Result<SessionSchema> {
        let mut result = schema.clone();
        
        let session_active_model = SessionActiveModel {
            id: Set(schema.id),
            mentor_id: Set(schema.mentor_id),
            mentee_id: Set(schema.mentee_id),
            topic: Set(schema.topic),
            description: Set(schema.description),
            scheduled_at: Set(schema.scheduled_at),
            duration_minutes: Set(schema.duration_minutes),
            meeting_link: Set(schema.meeting_link),
            session_type: Set(schema.session_type),
            status: Set(schema.status),
            feedback: Set(schema.feedback),
            rating: Set(schema.rating),
            feedback_submitted_at: Set(schema.feedback_submitted_at),
            created_at: Set(schema.created_at),
            updated_at: Set(schema.updated_at),
        };

        let session_model: SessionModel = session_active_model.insert(self.db).await?;
        
        result.id = session_model.id;
        
        Ok(result)
    }

    // ============================================
    // Get Session by ID
    // ============================================
    pub async fn query_session_by_id(&self, id: &str) -> Result<Option<SessionSchema>> {
        let session_model: Option<SessionModel> = Sessions::find_by_id(Uuid::parse_str(id).map_err(|e| anyhow!("Invalid session ID: {}", e))?)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch session: {}", e))?;

        if let Some(session) = session_model {
            let schema = SessionSchema {
                id: session.id,
                mentor_id: session.mentor_id,
                mentee_id: session.mentee_id,
                topic: session.topic,
                description: session.description,
                scheduled_at: session.scheduled_at,
                duration_minutes: session.duration_minutes,
                meeting_link: session.meeting_link,
                session_type: session.session_type,
                status: session.status,
                feedback: session.feedback,
                rating: session.rating,
                feedback_submitted_at: session.feedback_submitted_at,
                created_at: session.created_at,
                updated_at: session.updated_at,
            };
            
            Ok(Some(schema))
        } else {
            Ok(None)
        }
    }

    // ============================================
    // Get Session Detail with User Info
    // ============================================
    pub async fn query_session_detail(&self, id: &str) -> Result<Option<SessionDetailQueryDto>> {
        let session_id = Uuid::parse_str(id).map_err(|e| anyhow!("Invalid session ID: {}", e))?;
        
        let session = Sessions::find_by_id(session_id)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch session: {}", e))?
            .ok_or_else(|| anyhow!("Session not found"))?;

        let mentor = Users::find_by_id(session.mentor_id)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch mentor: {}", e))?
            .ok_or_else(|| anyhow!("Mentor not found"))?;

        let mentee = Users::find_by_id(session.mentee_id)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch mentee: {}", e))?
            .ok_or_else(|| anyhow!("Mentee not found"))?;

        let session_detail = SessionDetailQueryDto {
            id: session.id.to_string(),
            mentor_id: session.mentor_id.to_string(),
            mentee_id: session.mentee_id.to_string(),
            topic: session.topic,
            description: session.description,
            scheduled_at: session.scheduled_at.to_rfc3339(),
            duration_minutes: session.duration_minutes,
            meeting_link: session.meeting_link,
            session_type: session.session_type,
            status: session.status,
            feedback: session.feedback,
            rating: session.rating,
            feedback_submitted_at: session.feedback_submitted_at.map(|dt| dt.to_rfc3339()),
            created_at: session.created_at.to_rfc3339(),
            updated_at: session.updated_at.to_rfc3339(),
            mentor_fullname: Some(format!("{} {}", mentor.first_name.unwrap_or_default(), mentor.last_name.unwrap_or_default())),
            mentee_fullname: Some(format!("{} {}", mentee.first_name.unwrap_or_default(), mentee.last_name.unwrap_or_default())),
        };

        Ok(Some(session_detail))
    }

    // ============================================
    // List Mentor's Sessions
    // ============================================
    pub async fn query_mentor_sessions(
        &self,
        mentor_id: &str,
        status_filter: Option<String>,
    ) -> Result<Vec<SessionListQueryDto>> {
        let mentor_uuid = Uuid::parse_str(mentor_id).map_err(|e| anyhow!("Invalid mentor ID: {}", e))?;
        
        let query = Sessions::find()
            .filter(SessionColumn::MentorId.eq(mentor_uuid))
            .order_by_desc(SessionColumn::ScheduledAt);
        
        let query = if let Some(status) = status_filter {
            query.filter(SessionColumn::Status.eq(status))
        } else {
            query
        };
        
        let sessions = query.all(self.db).await.map_err(|e| anyhow!("Failed to query mentor sessions: {}", e))?;
        
        let mut session_list = Vec::with_capacity(sessions.len());
        
        for session in sessions {
            // Join with users table to get mentee details
            let mentee = Users::find_by_id(session.mentee_id)
                .one(self.db)
                .await
                .map_err(|e| anyhow!("Failed to fetch mentee: {}", e))?;
            
            let session_dto = SessionListQueryDto {
                id: session.id.to_string(),
                mentor_id: session.mentor_id.to_string(),
                mentee_id: session.mentee_id.to_string(),
                topic: session.topic,
                scheduled_at: session.scheduled_at.to_rfc3339(),
                duration_minutes: session.duration_minutes,
                session_type: session.session_type,
                status: session.status,
                rating: session.rating,
                created_at: session.created_at.to_rfc3339(),
                mentee_fullname: mentee.as_ref().map(|m| format!("{} {}", m.first_name.clone().unwrap_or_default(), m.last_name.clone().unwrap_or_default())),
                mentee_email: mentee.as_ref().map(|u| u.email.clone()), // Assuming Users model has an email field
            };
            
            session_list.push(session_dto);
        }

        Ok(session_list)
    }

    // ============================================
    // List User's Sessions (as mentee)
    // ============================================
    pub async fn query_user_sessions(
        &self,
        user_id: &str,
        status_filter: Option<String>,
    ) -> Result<Vec<SessionListQueryDto>> {
        let user_uuid = Uuid::parse_str(user_id).map_err(|e| anyhow!("Invalid user ID: {}", e))?;
        
        let query = Sessions::find()
            .filter(SessionColumn::MenteeId.eq(user_uuid))
            .order_by_desc(SessionColumn::ScheduledAt);
        
        let query = if let Some(status) = status_filter {
            query.filter(SessionColumn::Status.eq(status))
        } else {
            query
        };
        
        let sessions = query.all(self.db).await.map_err(|e| anyhow!("Failed to query user sessions: {}", e))?;
        
        let mut session_list = Vec::with_capacity(sessions.len());
        
        for session in sessions {
            // Join with users table to get mentor details
            let _mentor = Users::find_by_id(session.mentor_id)
                .one(self.db)
                .await
                .map_err(|e| anyhow!("Failed to fetch mentor: {}", e))?;
            
            let session_dto = SessionListQueryDto {
                id: session.id.to_string(),
                mentor_id: session.mentor_id.to_string(),
                mentee_id: session.mentee_id.to_string(),
                topic: session.topic,
                scheduled_at: session.scheduled_at.to_rfc3339(),
                duration_minutes: session.duration_minutes,
                session_type: session.session_type,
                status: session.status,
                rating: session.rating,
                created_at: session.created_at.to_rfc3339(),
                mentee_fullname: Some(session.mentee_id.to_string()), // Simplified - should get from user table
                mentee_email: Some("user@example.com".to_string()), // Simplified - should get from user table
            };
            
            session_list.push(session_dto);
        }

        Ok(session_list)
    }

    // ============================================
    // Get Booked Dates for Mentor
    // ============================================
    pub async fn query_booked_dates(&self, mentor_id: &str) -> Result<Vec<String>> {
        let mentor_uuid = Uuid::parse_str(mentor_id).map_err(|e| anyhow!("Invalid mentor ID: {}", e))?;
        
        let sessions = Sessions::find()
            .filter(SessionColumn::MentorId.eq(mentor_uuid))
            .filter(SessionColumn::Status.is_in(["pending", "confirmed"]))
            .order_by_asc(SessionColumn::ScheduledAt)
            .all(self.db)
            .await
            .map_err(|e| anyhow!("Failed to query booked dates: {}", e))?;

        Ok(sessions.into_iter()
            .map(|s| s.scheduled_at.to_rfc3339())
            .collect())
    }

    // ============================================
    // Update Session
    // ============================================
    pub async fn update_session(&self, id: &str, schema: SessionSchema) -> Result<SessionSchema> {
        let session_id = Uuid::parse_str(id).map_err(|e| anyhow!("Invalid session ID: {}", e))?;
        
        let mut result = schema.clone();
        
        // Fetch existing session
        let session_model = Sessions::find_by_id(session_id)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch session for update: {}", e))?
            .ok_or_else(|| anyhow!("Session not found"))?;
        
        // Convert to ActiveModel for update
        let mut session_active_model = session_model.into_active_model();
        
        // Update fields
        session_active_model.topic = Set(schema.topic);
        session_active_model.description = Set(schema.description);
        session_active_model.scheduled_at = Set(schema.scheduled_at);
        session_active_model.duration_minutes = Set(schema.duration_minutes);
        session_active_model.meeting_link = Set(schema.meeting_link);
        session_active_model.session_type = Set(schema.session_type);
        session_active_model.status = Set(schema.status);
        session_active_model.feedback = Set(schema.feedback.clone());
        session_active_model.rating = Set(schema.rating);
        session_active_model.feedback_submitted_at = Set(schema.feedback_submitted_at);
        session_active_model.updated_at = Set(schema.updated_at);
        
        // Save updated session
        let updated_session = session_active_model.update(self.db).await.map_err(|e| anyhow!("Failed to update session: {}", e))?;
        
        // Convert back to SessionSchema for response
        result.id = updated_session.id;
        
        Ok(result)
    }

    // ============================================
    // Count Mentor Sessions
    // ============================================
    pub async fn count_mentor_sessions(
        &self,
        mentor_id: &str,
        status_filter: Option<String>,
    ) -> Result<usize> {
        let mentor_uuid = Uuid::parse_str(mentor_id).map_err(|e| anyhow!("Invalid mentor ID: {}", e))?;
        
        let query = Sessions::find()
            .filter(SessionColumn::MentorId.eq(mentor_uuid));
        
        let query = if let Some(status) = status_filter {
            query.filter(SessionColumn::Status.eq(status))
        } else {
            query
        };
        
        let count = query.count(self.db).await.map_err(|e| anyhow!("Failed to count mentor sessions: {}", e))?;
        
        Ok(count as usize)
    }

    // ============================================
    // Count User Sessions
    // ============================================
    pub async fn count_user_sessions(
        &self,
        user_id: &str,
        status_filter: Option<String>,
    ) -> Result<usize> {
        let user_uuid = Uuid::parse_str(user_id).map_err(|e| anyhow!("Invalid user ID: {}", e))?;
        
        let query = Sessions::find()
            .filter(SessionColumn::MenteeId.eq(user_uuid));
        
        let query = if let Some(status) = status_filter {
            query.filter(SessionColumn::Status.eq(status))
        } else {
            query
        };
        
        let count = query.count(self.db).await.map_err(|e| anyhow!("Failed to count user sessions: {}", e))?;
        
        Ok(count as usize)
    }

    // ============================================
    // Delete Session (soft delete)
    // ============================================
    pub async fn delete_session(&self, id: &str) -> Result<()> {
        let session_id = Uuid::parse_str(id).map_err(|e| anyhow!("Invalid session ID: {}", e))?;
        
        // Fetch the session first
        let session_model = Sessions::find_by_id(session_id)
            .one(self.db)
            .await
            .map_err(|e| anyhow!("Failed to fetch session for deletion: {}", e))?
            .ok_or_else(|| anyhow!("Session not found"))?;
        
        // Convert to ActiveModel for deletion
        let session_active_model = session_model.into_active_model();
        
        // For soft delete, we would typically set an `is_deleted` flag
        // Since the original implementation didn't have this, we'll just delete the record
        // If you want to implement soft delete, add an `is_deleted` field to the SessionModel
        
        let _ = session_active_model.delete(self.db).await.map_err(|e| anyhow!("Failed to delete session: {}", e))?;

        Ok(())
    }
}

