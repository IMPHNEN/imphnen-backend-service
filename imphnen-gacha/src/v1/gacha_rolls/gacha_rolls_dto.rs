use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;
use crate::v1::gacha_items::GachaItemDto;
use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct GachaRollRequestDto {
	#[validate(length(min = 1, max = 100, message = "Item ID must be between 1 and 100 characters"))]
	pub item_id: String,

	#[validate(range(min = 0.0, max = 1.0, message = "Weight must be between 0.0 and 1.0"))]
	pub weight: f32,

	#[validate(range(min = 1, max = 100, message = "Quantity must be between 1 and 100"))]
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
			id: dto.id.clone(),
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
	pub id: String,
	// item can be missing in the DB (during partial queries); make optional to allow graceful handling
	pub item: Option<GachaItemSchema>,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}
