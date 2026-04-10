use crate::roadmap::domain::roadmap::RoadmapEntity;
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RoadmapCreateRequestDto {
	pub title: String,
	pub description: String,
	#[schema(example = "upcoming")]
	pub status: String,
}

impl ZodValidate for RoadmapCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

impl From<RoadmapCreateRequestDto> for RoadmapEntity {
	fn from(dto: RoadmapCreateRequestDto) -> Self {
		RoadmapEntity {
			id: Uuid::new_v4(),
			title: dto.title,
			description: dto.description,
			status: dto.status,
			votes: 0,
			is_deleted: false,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RoadmapUpdateRequestDto {
	pub title: String,
	pub description: String,
	#[schema(example = "upcoming")]
	pub status: String,
}

impl ZodValidate for RoadmapUpdateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RoadmapListItemDto {
	pub id: String,
	pub title: String,
	pub description: String,
	pub status: String,
	pub votes: i32,
	pub is_deleted: bool,
	pub created_at: String,
}

impl From<RoadmapEntity> for RoadmapListItemDto {
	fn from(e: RoadmapEntity) -> Self {
		RoadmapListItemDto {
			id: e.id.to_string(),
			title: e.title,
			description: e.description,
			status: e.status,
			votes: e.votes,
			is_deleted: e.is_deleted,
			created_at: e.created_at.to_rfc3339(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RoadmapDetailItemDto {
	pub id: String,
	pub title: String,
	pub description: String,
	pub status: String,
	pub votes: i32,
	pub created_at: String,
	pub updated_at: String,
}

impl From<RoadmapEntity> for RoadmapDetailItemDto {
	fn from(e: RoadmapEntity) -> Self {
		RoadmapDetailItemDto {
			id: e.id.to_string(),
			title: e.title,
			description: e.description,
			status: e.status,
			votes: e.votes,
			created_at: e.created_at.to_rfc3339(),
			updated_at: e.updated_at.to_rfc3339(),
		}
	}
}
