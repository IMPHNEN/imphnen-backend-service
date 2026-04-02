use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct SessionEntity {
    pub id: Uuid,
    pub mentor_id: Uuid,
    pub mentee_id: Uuid,
    pub topic: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub meeting_link: Option<String>,
    pub session_type: String,
    pub status: String,
    pub feedback: Option<String>,
    pub rating: Option<i32>,
    pub feedback_submitted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
