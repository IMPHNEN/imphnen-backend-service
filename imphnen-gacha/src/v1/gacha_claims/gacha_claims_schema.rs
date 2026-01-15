use crate::v1::gacha_rolls::gacha_rolls_dto::GachaRollQueryDto;
use crate::{make_thing};
use imphnen_iam::get_iso_date;
use imphnen_entities::ResourceEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::v1::gacha_claims::gacha_claims_dto::GachaClaimRequestDto;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaClaimSchema {
	pub id: String,
	pub user: String,
	pub item: String,
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

	pub fn roll(roll: GachaRollQueryDto, user_id: String) -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::GachaClaims.to_string(),
				&Uuid::new_v4().to_string(),
			),
			user: user_id,
			// roll.item is optional at the DTO level; assume caller ensured a valid item exists
			item: roll
				.item
				.as_ref()
				.map(|i| i.id.clone())
				.unwrap_or_else(|| make_thing(&ResourceEnum::GachaItems.to_string(), &Uuid::new_v4().to_string())),
			..Default::default()
		}
	}
}
