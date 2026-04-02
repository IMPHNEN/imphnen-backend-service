use crate::gacha_rolls::domain::gacha_roll::GachaRollEntity;
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollCreateRequestDto {
	pub item_id: String,
	pub weight: f32,
	pub quantity: i32,
}

impl ZodValidate for GachaRollCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaRollItemDto {
	pub id: String,
	pub user_id: String,
	pub gacha_id: String,
	pub item_id: String,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl From<&GachaRollEntity> for GachaRollItemDto {
	fn from(e: &GachaRollEntity) -> Self {
		GachaRollItemDto {
			id: e.id.to_string(),
			user_id: e.user_id.to_string(),
			gacha_id: e.gacha_id.clone(),
			item_id: e.item_id.to_string(),
			weight: e.weight,
			quantity: e.quantity,
			is_deleted: e.is_deleted,
			created_at: e.created_at.map(|d| d.to_string()),
			updated_at: e.updated_at.map(|d| d.to_string()),
		}
	}
}
