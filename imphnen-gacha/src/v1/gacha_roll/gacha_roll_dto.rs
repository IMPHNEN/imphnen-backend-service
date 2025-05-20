use crate::{GachaItemDto, GachaItemSchema};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaRollRequestDto {
	#[validate(length(min = 1, message = "Item ID must not be empty"))]
	pub item_id: String,
	#[validate(range(min = 1, message = "Weight must be greater than zero"))]
	pub weight: f32,
	#[validate(range(min = 1, message = "Quantity must be at least 1"))]
	pub quantity: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollItemDto {
	pub id: String,
	pub item: GachaItemDto,
	pub weight: String,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl GachaRollItemDto {
	pub fn from(dto: &GachaRollQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			item: GachaItemDto::from(dto.item.clone()),
			weight: dto.weight.to_string(),
			quantity: dto.quantity,
			is_deleted: dto.is_deleted,
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaRollQueryDto {
	pub id: Thing,
	pub item: GachaItemSchema,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
