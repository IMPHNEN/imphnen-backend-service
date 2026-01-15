use serde::{Deserialize, Serialize};
use uuid::Uuid;
use imphnen_utils::{get_iso_date};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GachaCreditSchema {
	pub id: String,
	pub user: String,
	pub available_rolls: i32,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for GachaCreditSchema {
	fn default() -> Self {
		GachaCreditSchema {
			id: Uuid::new_v4().to_string(),
			user: Uuid::new_v4().to_string(),
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
