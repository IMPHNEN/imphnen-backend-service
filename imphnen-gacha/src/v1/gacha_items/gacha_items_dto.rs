use super::GachaItemSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemRequestDto {
	#[validate(length(min = 1, message = "Item name must not be empty"))]
	pub name: String,
	#[validate(length(min = 1, message = "Image URL must not be empty"))]
	pub image_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaItemUpdateRequestDto {
	#[validate(length(min = 1, message = "Item name must not be empty"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[validate(length(min = 1, message = "Image URL must not be empty"))]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub image_url: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl GachaItemDto {
	pub fn from(dto: GachaItemSchema) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			name: dto.name,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
		}
	}
}
