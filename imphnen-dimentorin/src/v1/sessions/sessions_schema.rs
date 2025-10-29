use super::{BookSessionRequestDto, SessionFeedbackRequestDto, UpdateSessionStatusRequestDto};
use imphnen_libs::ResourceEnum;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionSchema {
    pub id: Thing,
    pub mentor_id: Thing,
    pub mentee_id: Thing,
    pub topic: String,
    pub description: Option<String>,
    pub scheduled_at: String, // ISO 8601 datetime
    pub duration_minutes: i32,
    pub meeting_link: Option<String>,
    pub session_type: String, // "video_call", "phone_call", "chat"
    pub status: String, // "pending", "confirmed", "completed", "cancelled", "no_show"
    pub feedback: Option<String>,
    pub rating: Option<i32>, // 1-5
    pub feedback_submitted_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl Default for SessionSchema {
    fn default() -> Self {
        Self {
            id: make_thing(
                ResourceEnum::Sessions.to_string().as_str(),
                &Uuid::new_v4().to_string(),
            ),
            mentor_id: make_thing(
                ResourceEnum::Mentors.to_string().as_str(),
                &Uuid::new_v4().to_string(),
            ),
            mentee_id: make_thing(
                ResourceEnum::Users.to_string().as_str(),
                &Uuid::new_v4().to_string(),
            ),
            topic: String::new(),
            description: None,
            scheduled_at: get_iso_date(),
            duration_minutes: 60,
            meeting_link: None,
            session_type: "video_call".to_string(),
            status: "pending".to_string(),
            feedback: None,
            rating: None,
            feedback_submitted_at: None,
            created_at: get_iso_date(),
            updated_at: get_iso_date(),
        }
    }
}

impl SessionSchema {
    pub fn from_book_request(
        mentor_id: Thing,
        mentee_id: Thing,
        request: BookSessionRequestDto,
    ) -> Self {
        Self {
            mentor_id,
            mentee_id,
            topic: request.topic,
            description: request.description,
            scheduled_at: request.scheduled_at,
            duration_minutes: request.duration_minutes.unwrap_or(60),
            session_type: request.session_type.unwrap_or_else(|| "video_call".to_string()),
            ..Default::default()
        }
    }

    pub fn update_status(&mut self, request: UpdateSessionStatusRequestDto) {
        self.status = request.status;
        if let Some(link) = request.meeting_link {
            self.meeting_link = Some(link);
        }
        self.updated_at = get_iso_date();
    }

    pub fn add_feedback(&mut self, request: SessionFeedbackRequestDto) {
        self.feedback = Some(request.feedback);
        self.rating = Some(request.rating);
        self.feedback_submitted_at = Some(get_iso_date());
        self.updated_at = get_iso_date();
    }
}
