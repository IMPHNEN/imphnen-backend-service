use crate::events::domain::event::EventEntity;
use chrono::{DateTime, Utc};
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsCreateRequestDto {
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub end_date: DateTime<Utc>,
	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub start_date: DateTime<Utc>,
	pub location: Option<String>,
	pub is_online: bool,
}

impl ZodValidate for EventsCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

impl From<EventsCreateRequestDto> for EventEntity {
	fn from(dto: EventsCreateRequestDto) -> Self {
		EventEntity {
			id: Uuid::new_v4(),
			name: dto.name,
			description: dto.description,
			detail_link: dto.detail_link,
			price: dto.price,
			is_online: dto.is_online,
			is_deleted: false,
			location: dto.location,
			start_date: dto.start_date,
			end_date: dto.end_date,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsUpdateRequestDto {
	pub name: String,
	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub end_date: DateTime<Utc>,
	#[schema(example = "2025-09-20T13:00:00Z", value_type = String)]
	pub start_date: DateTime<Utc>,
	pub price: f64,
	pub is_online: bool,
	pub description: String,
	pub detail_link: String,
	pub location: Option<String>,
}

impl ZodValidate for EventsUpdateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsListItemDto {
	pub id: String,
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	pub is_online: bool,
	pub start_date: String,
	pub end_date: String,
	pub created_at: String,
	pub location: Option<String>,
	pub is_deleted: bool,
}

impl From<EventEntity> for EventsListItemDto {
	fn from(e: EventEntity) -> Self {
		EventsListItemDto {
			id: e.id.to_string(),
			name: e.name,
			description: e.description,
			detail_link: e.detail_link,
			price: e.price,
			is_online: e.is_online,
			start_date: e.start_date.to_rfc3339(),
			end_date: e.end_date.to_rfc3339(),
			created_at: e.created_at.to_rfc3339(),
			location: e.location,
			is_deleted: e.is_deleted,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct EventsDetailItemDto {
	pub id: String,
	pub name: String,
	pub description: String,
	pub detail_link: String,
	pub price: f64,
	pub is_online: bool,
	pub start_date: String,
	pub end_date: String,
	pub created_at: String,
	pub updated_at: String,
	pub location: Option<String>,
}

impl From<EventEntity> for EventsDetailItemDto {
	fn from(e: EventEntity) -> Self {
		EventsDetailItemDto {
			id: e.id.to_string(),
			name: e.name,
			description: e.description,
			detail_link: e.detail_link,
			price: e.price,
			is_online: e.is_online,
			start_date: e.start_date.to_rfc3339(),
			end_date: e.end_date.to_rfc3339(),
			created_at: e.created_at.to_rfc3339(),
			updated_at: e.updated_at.to_rfc3339(),
			location: e.location,
		}
	}
}
