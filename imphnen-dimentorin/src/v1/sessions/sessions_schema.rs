use super::{BookSessionRequestDto, SessionFeedbackRequestDto, UpdateSessionStatusRequestDto};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use imphnen_entities::error_dto::error::Error;

// Type alias for Thing
pub type Thing = String;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSchema {
    pub id: Uuid,
    pub mentor_id: Uuid,
    pub mentee_id: Uuid,
    pub topic: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub meeting_link: Option<String>,
    pub session_type: String, // "video_call", "phone_call", "chat"
    pub status: String, // "pending", "confirmed", "completed", "cancelled", "no_show"
    pub feedback: Option<String>,
    pub rating: Option<i32>, // 1-5
    pub feedback_submitted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Default for SessionSchema {
    fn default() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            mentor_id: Uuid::new_v4(),
            mentee_id: Uuid::new_v4(),
            topic: String::new(),
            description: None,
            scheduled_at: now,
            duration_minutes: 60,
            meeting_link: None,
            session_type: "video_call".to_string(),
            status: "pending".to_string(),
            feedback: None,
            rating: None,
            feedback_submitted_at: None,
            created_at: now,
            updated_at: now,
        }
    }
}

impl SessionSchema {
    pub fn from_book_request(
        mentor_id: Thing,
        mentee_id: Thing,
        request: BookSessionRequestDto,
    ) -> Result<Self, Error> {
        let scheduled_at = DateTime::parse_from_rfc3339(&request.scheduled_at)
            .map_err(|e| Error::Validation(format!("Invalid scheduled_at format: {}", e)))?
            .with_timezone(&Utc);
        
        let mentor_id = Uuid::parse_str(&mentor_id)
            .map_err(|e| Error::Validation(format!("Invalid mentor_id: {}", e)))?;
        let mentee_id = Uuid::parse_str(&mentee_id)
            .map_err(|e| Error::Validation(format!("Invalid mentee_id: {}", e)))?;
        
        Ok(Self {
            mentor_id,
            mentee_id,
            topic: request.topic,
            description: request.description,
            scheduled_at,
            duration_minutes: request.duration_minutes.unwrap_or(60),
            session_type: request.session_type.unwrap_or_else(|| "video_call".to_string()),
            ..Default::default()
        })
    }

    pub fn update_status(&mut self, request: UpdateSessionStatusRequestDto) {
        self.status = request.status;
        if let Some(link) = request.meeting_link {
            self.meeting_link = Some(link);
        }
        self.updated_at = Utc::now();
    }

    pub fn add_feedback(&mut self, request: SessionFeedbackRequestDto) {
        self.feedback = Some(request.feedback);
        self.rating = Some(request.rating);
        self.feedback_submitted_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
}
