use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// ============================================
// Book Session (POST /v1/mentors/{id}/sessions/book)
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct BookSessionRequestDto {
    #[validate(length(min = 3, max = 200, message = "Topic must be 3-200 characters"))]
    pub topic: String,
    
    #[validate(length(max = 1000, message = "Description must be max 1000 characters"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    #[validate(length(min = 1, message = "Scheduled time is required"))]
    pub scheduled_at: String, // ISO 8601 datetime
    
    #[validate(range(min = 15, max = 240, message = "Duration must be 15-240 minutes"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i32>,
    
    #[validate(length(max = 50, message = "Session type must be max 50 characters"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<String>, // "video_call", "phone_call", "chat"
}

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

// ============================================
// List Sessions (GET /v1/mentors/{id}/sessions & /v1/users/me/sessions)
// ============================================

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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionListResponseDto {
    pub sessions: Vec<SessionListItemDto>,
    pub total: usize,
}

// ============================================
// Session Detail
// ============================================

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

// ============================================
// Mentor Availability (GET /v1/mentors/{id}/availability)
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AvailabilitySlotDto {
    pub date: String, // YYYY-MM-DD
    pub time: String, // HH:MM
    pub available: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorAvailabilityDto {
    pub mentor_id: String,
    pub availability_commitment: String,
    pub preferred_formats: Vec<String>,
    pub slots: Vec<AvailabilitySlotDto>,
    pub booked_dates: Vec<String>, // Dates with existing sessions
}

// ============================================
// Update Session Status (PUT /v1/sessions/{id}/status)
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateSessionStatusRequestDto {
    #[validate(length(min = 1, max = 50, message = "Status must be 1-50 characters"))]
    pub status: String, // "confirmed", "completed", "cancelled", "no_show"
    
    #[validate(url(message = "Meeting link must be a valid URL"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_link: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSessionStatusResponseDto {
    pub id: String,
    pub status: String,
    pub meeting_link: Option<String>,
    pub updated_at: String,
}

// ============================================
// Submit Feedback (POST /v1/sessions/{id}/feedback)
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct SessionFeedbackRequestDto {
    #[validate(length(min = 10, max = 2000, message = "Feedback must be 10-2000 characters"))]
    pub feedback: String,
    
    #[validate(range(min = 1, max = 5, message = "Rating must be 1-5"))]
    pub rating: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionFeedbackResponseDto {
    pub id: String,
    pub feedback: String,
    pub rating: i32,
    pub submitted_at: String,
}

// ============================================
// Query DTOs (internal use)
// ============================================

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionDetailQueryDto {
    pub id: String,
    pub mentor_id: String,
    pub mentee_id: String,
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
    pub mentor_fullname: Option<String>,
    pub mentee_fullname: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionListQueryDto {
    pub id: String,
    pub mentor_id: String,
    pub mentee_id: String,
    pub topic: String,
    pub scheduled_at: String,
    pub duration_minutes: i32,
    pub session_type: String,
    pub status: String,
    pub rating: Option<i32>,
    pub created_at: String,
    pub mentee_fullname: Option<String>,
    pub mentee_email: Option<String>,
}
