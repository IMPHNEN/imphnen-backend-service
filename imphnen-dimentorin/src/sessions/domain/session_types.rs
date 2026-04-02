pub struct BookSessionCommand {
	pub topic: String,
	pub description: Option<String>,
	pub scheduled_at: String,
	pub duration_minutes: Option<i32>,
	pub session_type: Option<String>,
}

pub struct BookedSession {
	pub id: String,
	pub mentor_id: String,
	pub mentee_id: String,
	pub topic: String,
	pub description: Option<String>,
	pub scheduled_at: String,
	pub duration_minutes: i32,
	pub session_type: String,
	pub status: String,
	pub created_at: String,
}

pub struct SessionListItem {
	pub id: String,
	pub mentor_id: String,
	pub mentee_id: String,
	pub mentee_fullname: Option<String>,
	pub mentee_email: Option<String>,
	pub topic: String,
	pub scheduled_at: String,
	pub duration_minutes: i32,
	pub session_type: String,
	pub status: String,
	pub rating: Option<i32>,
	pub created_at: String,
}

pub struct SessionList {
	pub sessions: Vec<SessionListItem>,
	pub total: usize,
}

pub struct SessionDetail {
	pub id: String,
	pub mentor_id: String,
	pub mentor_fullname: Option<String>,
	pub mentee_id: String,
	pub mentee_fullname: Option<String>,
	pub topic: String,
	pub description: Option<String>,
	pub scheduled_at: String,
	pub duration_minutes: i32,
	pub meeting_link: Option<String>,
	pub session_type: String,
	pub status: String,
	pub feedback: Option<String>,
	pub rating: Option<i32>,
	pub feedback_submitted_at: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

pub struct AvailabilitySlot {
	pub date: String,
	pub time: String,
	pub available: bool,
}

pub struct MentorAvailability {
	pub mentor_id: String,
	pub availability_commitment: String,
	pub preferred_formats: Vec<String>,
	pub slots: Vec<AvailabilitySlot>,
	pub booked_dates: Vec<String>,
}

pub struct UpdateSessionStatusCommand {
	pub status: String,
	pub meeting_link: Option<String>,
}

pub struct UpdatedSessionStatus {
	pub id: String,
	pub status: String,
	pub meeting_link: Option<String>,
	pub updated_at: String,
}

pub struct SessionFeedbackCommand {
	pub feedback: String,
	pub rating: i32,
}

pub struct SessionFeedbackResult {
	pub id: String,
	pub feedback: String,
	pub rating: i32,
	pub submitted_at: String,
}
