use crate::sessions::domain::{
	BookSessionCommand, SessionFeedbackCommand, UpdateSessionStatusCommand,
};
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use zod_rs::prelude::*;

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

impl From<BookSessionRequestDto> for BookSessionCommand {
	fn from(dto: BookSessionRequestDto) -> Self {
		Self {
			topic: dto.topic,
			description: dto.description,
			scheduled_at: dto.scheduled_at,
			duration_minutes: dto.duration_minutes,
			session_type: dto.session_type,
		}
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

impl From<UpdateSessionStatusRequestDto> for UpdateSessionStatusCommand {
	fn from(dto: UpdateSessionStatusRequestDto) -> Self {
		Self {
			status: dto.status,
			meeting_link: dto.meeting_link,
		}
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

impl From<SessionFeedbackRequestDto> for SessionFeedbackCommand {
	fn from(dto: SessionFeedbackRequestDto) -> Self {
		Self {
			feedback: dto.feedback,
			rating: dto.rating,
		}
	}
}
