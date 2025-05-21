use imphnen_iam::get_iso_date;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaCreditSchema {
	pub id: Thing,
	pub user: Thing,
	pub available_rolls: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaCreditSchema {
	fn default() -> Self {
		GachaCreditSchema {
			id: Thing::from(("app_gacha_credits", "uuid")),
			user: Thing::from(("app_users", "uuid")),
			available_rolls: 0,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}

impl GachaCreditSchema {
	pub fn from(&self) -> Self {
		Self {
			id: self.id.clone(),
			user: self.user.clone(),
			available_rolls: self.available_rolls,
			is_deleted: false,
			created_at: Some(get_iso_date()),
			updated_at: Some(get_iso_date()),
		}
	}
}
