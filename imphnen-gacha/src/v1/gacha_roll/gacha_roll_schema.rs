use crate::{ResourceEnum, make_thing};
use imphnen_iam::get_iso_date;
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

use super::GachaRollRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaRollSchema {
	pub id: Thing,
	pub item: Thing,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaRollSchema {
	fn default() -> Self {
		GachaRollSchema {
			id: make_thing(
				&ResourceEnum::GachaRolls.to_string(),
				&Uuid::new_v4().to_string(),
			),
			item: make_thing(
				&ResourceEnum::GachaItems.to_string(),
				&Uuid::new_v4().to_string(),
			),
			weight: 0.0,
			quantity: 0,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}

impl GachaRollSchema {
	pub fn create(dto: GachaRollRequestDto) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::GachaRolls.to_string(),
				&Uuid::new_v4().to_string(),
			),
			item: make_thing(&ResourceEnum::GachaItems.to_string(), &dto.item_id),
			weight: dto.weight,
			quantity: dto.quantity,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}
