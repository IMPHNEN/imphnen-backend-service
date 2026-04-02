use crate::sessions::domain::{
	AvailabilitySlot, BookedSession, MentorAvailability, SessionDetail,
	SessionFeedbackResult, SessionList, SessionListItem, UpdatedSessionStatus,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct BookSessionResponseDto {
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

impl From<BookedSession> for BookSessionResponseDto {
	fn from(s: BookedSession) -> Self {
		Self {
			id: s.id,
			mentor_id: s.mentor_id,
			mentee_id: s.mentee_id,
			topic: s.topic,
			description: s.description,
			scheduled_at: s.scheduled_at,
			duration_minutes: s.duration_minutes,
			session_type: s.session_type,
			status: s.status,
			created_at: s.created_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionListItemDto {
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

impl From<SessionListItem> for SessionListItemDto {
	fn from(s: SessionListItem) -> Self {
		Self {
			id: s.id,
			mentor_id: s.mentor_id,
			mentee_id: s.mentee_id,
			mentee_fullname: s.mentee_fullname,
			mentee_email: s.mentee_email,
			topic: s.topic,
			scheduled_at: s.scheduled_at,
			duration_minutes: s.duration_minutes,
			session_type: s.session_type,
			status: s.status,
			rating: s.rating,
			created_at: s.created_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionListResponseDto {
	pub sessions: Vec<SessionListItemDto>,
	pub total: usize,
}

impl From<SessionList> for SessionListResponseDto {
	fn from(list: SessionList) -> Self {
		Self {
			sessions: list
				.sessions
				.into_iter()
				.map(SessionListItemDto::from)
				.collect(),
			total: list.total,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionDetailDto {
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

impl From<SessionDetail> for SessionDetailDto {
	fn from(d: SessionDetail) -> Self {
		Self {
			id: d.id,
			mentor_id: d.mentor_id,
			mentor_fullname: d.mentor_fullname,
			mentee_id: d.mentee_id,
			mentee_fullname: d.mentee_fullname,
			topic: d.topic,
			description: d.description,
			scheduled_at: d.scheduled_at,
			duration_minutes: d.duration_minutes,
			meeting_link: d.meeting_link,
			session_type: d.session_type,
			status: d.status,
			feedback: d.feedback,
			rating: d.rating,
			feedback_submitted_at: d.feedback_submitted_at,
			created_at: d.created_at,
			updated_at: d.updated_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AvailabilitySlotDto {
	pub date: String,
	pub time: String,
	pub available: bool,
}

impl From<AvailabilitySlot> for AvailabilitySlotDto {
	fn from(s: AvailabilitySlot) -> Self {
		Self {
			date: s.date,
			time: s.time,
			available: s.available,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorAvailabilityDto {
	pub mentor_id: String,
	pub availability_commitment: String,
	pub preferred_formats: Vec<String>,
	pub slots: Vec<AvailabilitySlotDto>,
	pub booked_dates: Vec<String>,
}

impl From<MentorAvailability> for MentorAvailabilityDto {
	fn from(a: MentorAvailability) -> Self {
		Self {
			mentor_id: a.mentor_id,
			availability_commitment: a.availability_commitment,
			preferred_formats: a.preferred_formats,
			slots: a.slots.into_iter().map(AvailabilitySlotDto::from).collect(),
			booked_dates: a.booked_dates,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSessionStatusResponseDto {
	pub id: String,
	pub status: String,
	pub meeting_link: Option<String>,
	pub updated_at: String,
}

impl From<UpdatedSessionStatus> for UpdateSessionStatusResponseDto {
	fn from(u: UpdatedSessionStatus) -> Self {
		Self {
			id: u.id,
			status: u.status,
			meeting_link: u.meeting_link,
			updated_at: u.updated_at,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionFeedbackResponseDto {
	pub id: String,
	pub feedback: String,
	pub rating: i32,
	pub submitted_at: String,
}

impl From<SessionFeedbackResult> for SessionFeedbackResponseDto {
	fn from(r: SessionFeedbackResult) -> Self {
		Self {
			id: r.id,
			feedback: r.feedback,
			rating: r.rating,
			submitted_at: r.submitted_at,
		}
	}
}
