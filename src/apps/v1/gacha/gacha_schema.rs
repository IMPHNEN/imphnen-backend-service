use crate::{make_thing, ResourceEnum};
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GachaItemSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaItemSchema {
	fn default() -> Self {
		GachaItemSchema {
			id: make_thing(
				&ResourceEnum::GachaItems.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}

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
			created_at: None,
			updated_at: None,
		}
	}
}

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
			weight: 0.2,
			quantity: 2,
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}
