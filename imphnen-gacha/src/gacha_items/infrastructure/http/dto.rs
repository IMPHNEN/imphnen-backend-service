use crate::gacha_items::domain::gacha_item::GachaItemEntity;
use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemCreateRequestDto {
	pub item_code: String,
	pub name: String,
	pub description: String,
	pub rarity: String,
	pub type_: String,
	pub category: String,
	pub value: i32,
	pub weight: f64,
	pub stock: i32,
	pub is_limited: bool,
	pub metadata: Option<Value>,
}

impl ZodValidate for GachaItemCreateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

impl From<GachaItemCreateRequestDto> for GachaItemEntity {
	fn from(dto: GachaItemCreateRequestDto) -> Self {
		GachaItemEntity {
			id: Uuid::new_v4(),
			item_code: dto.item_code,
			name: dto.name,
			description: dto.description,
			rarity: dto.rarity,
			type_: dto.type_,
			category: dto.category,
			value: dto.value,
			weight: dto.weight,
			stock: dto.stock,
			is_limited: dto.is_limited,
			metadata: dto.metadata,
			is_deleted: false,
			created_at: chrono::Utc::now(),
			updated_at: chrono::Utc::now(),
			deleted_at: None,
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemUpdateRequestDto {
	pub item_code: String,
	pub name: String,
	pub description: String,
	pub rarity: String,
	pub type_: String,
	pub category: String,
	pub value: i32,
	pub weight: f64,
	pub stock: i32,
	pub is_limited: bool,
	pub metadata: Option<Value>,
}

impl ZodValidate for GachaItemUpdateRequestDto {
	fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
		serde_json::from_value(value.clone()).map_err(|e| e.to_string())
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaItemDto {
	pub id: String,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl From<GachaItemEntity> for GachaItemDto {
	fn from(e: GachaItemEntity) -> Self {
		GachaItemDto {
			id: e.id.to_string(),
			name: e.name,
			is_deleted: e.is_deleted,
			created_at: Some(e.created_at.to_rfc3339()),
			updated_at: Some(e.updated_at.to_rfc3339()),
		}
	}
}
