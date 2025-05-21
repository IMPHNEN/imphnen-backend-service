use crate::{GachaRollQueryDto, ResourceEnum, make_thing};
use imphnen_iam::get_iso_date;
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

use super::GachaClaimRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaClaimSchema {
	pub id: Thing,
	pub user: Thing,
	pub item: Thing,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaClaimSchema {
	fn default() -> Self {
		GachaClaimSchema {
			id: make_thing(
				&ResourceEnum::GachaClaims.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			item: make_thing(
				&ResourceEnum::GachaItems.to_string(),
				&Uuid::new_v4().to_string(),
			),
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}

impl GachaClaimSchema {
	pub fn from(dto: GachaClaimRequestDto) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::GachaClaims.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: make_thing(&ResourceEnum::Users.to_string(), &dto.user_id),
			item: make_thing(&ResourceEnum::GachaItems.to_string(), &dto.item_id),
			..Default::default()
		}
	}

	pub fn roll(roll: GachaRollQueryDto, user_id: Thing) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::GachaClaims.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: user_id,
			item: roll.item.id.clone(),
			..Default::default()
		}
	}
}
