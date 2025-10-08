use crate::v1::gacha_items::GachaItemDto;
use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaRollRequestDto {
	#[validate(length(min = 1, message = "Item ID must not be empty"))]
	pub item_id: String,
	pub weight: f32,
	#[validate(range(min = 1, message = "Quantity must be at least 1"))]
	pub quantity: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollItemDto {
	pub id: String,
	pub item: GachaItemDto,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl GachaRollItemDto {
	pub fn from(dto: &GachaRollQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw(),
			// Handle case where item might be missing
			item: match &dto.item {
				Some(item) => GachaItemDto::from(item.clone()),
				None => GachaItemDto {
					id: "".to_string(),
					name: "Unknown".to_string(),
					is_deleted: false,
					created_at: None,
					updated_at: None,
				}
			},
			weight: dto.weight,
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
	// item can be missing in the DB (during partial queries); make optional to allow graceful handling
	pub item: Option<GachaItemSchema>,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
