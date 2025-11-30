use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::v1::gacha_rolls::gacha_rolls_dto::GachaRollRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaRollSchema {
	pub id: String,
	pub user_id: String,
	pub gacha_id: String,
	pub item_id: String,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

impl Default for GachaRollSchema {
	fn default() -> Self {
		GachaRollSchema {
			id: Uuid::new_v4().to_string(),
			user_id: "".to_string(),
			gacha_id: "".to_string(),
			item_id: "".to_string(),
			weight: 0.0,
			quantity: 0,
			is_deleted: false,
			created_at: Some(Utc::now()),
			updated_at: Some(Utc::now()),
		}
	}
}

impl GachaRollSchema {
	pub fn create(dto: GachaRollRequestDto, user_id: String, gacha_id: String) -> Self {
		Self {
			id: Uuid::new_v4().to_string(),
			user_id,
			gacha_id,
			item_id: dto.item_id,
			weight: dto.weight,
			quantity: dto.quantity,
			..Default::default()
		}
	}
}
