use crate::{ResourceEnum, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

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
