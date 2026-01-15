use serde::{Deserialize, Serialize};
use uuid::Uuid;
use serde_json::Value;

use crate::v1::gacha_items::gacha_items_dto::GachaItemRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaItemSchema {
	pub id: String,
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
	pub image_url: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaItemSchema {
	fn default() -> Self {
		GachaItemSchema {
			id: Uuid::new_v4().to_string(),
			item_code: String::new(),
			name: String::new(),
			description: String::new(),
			rarity: String::new(),
			type_: String::new(),
			category: String::new(),
			value: 0,
			weight: 0.0,
			stock: 0,
			is_limited: false,
			metadata: None,
			image_url: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}

impl GachaItemSchema {
	pub fn from(dto: GachaItemRequestDto) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
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
			image_url: dto.image_url,
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}

	pub fn from_existing(existing: GachaItemSchema) -> Self {
		existing
	}
}
