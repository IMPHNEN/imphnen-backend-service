use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

// ============================================================
// Request DTOs
// ============================================================

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct BookSessionRequestDto {
    #[zod(min_length(3), max_length(200))]
    pub topic: String,
    #[zod(max_length(1000))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[zod(min_length(1))]
    pub scheduled_at: String,
    #[zod(min(15.0), max(240.0), int)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_minutes: Option<i32>,
    #[zod(max_length(50))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_type: Option<String>,
}

impl ZodValidate for BookSessionRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct UpdateSessionStatusRequestDto {
    #[zod(min_length(1), max_length(50))]
    pub status: String,
    #[zod(url)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meeting_link: Option<String>,
}

impl ZodValidate for UpdateSessionStatusRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, ZodSchema)]
pub struct SessionFeedbackRequestDto {
    #[zod(min_length(10), max_length(2000))]
    pub feedback: String,
    #[zod(min(1.0), max(5.0), int)]
    pub rating: i32,
}

impl ZodValidate for SessionFeedbackRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        Self::validate_and_parse(value).map_err(|e| e.to_string())
    }
}

// ============================================================
// Response DTOs
// ============================================================

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

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct AvailabilitySlotDto {
    pub date: String,
    pub time: String,
    pub available: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct MentorAvailabilityDto {
    pub mentor_id: String,
    pub availability_commitment: String,
    pub preferred_formats: Vec<String>,
    pub slots: Vec<AvailabilitySlotDto>,
    pub booked_dates: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateSessionStatusResponseDto {
    pub id: String,
    pub status: String,
    pub meeting_link: Option<String>,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct SessionFeedbackResponseDto {
    pub id: String,
    pub feedback: String,
    pub rating: i32,
    pub submitted_at: String,
}
