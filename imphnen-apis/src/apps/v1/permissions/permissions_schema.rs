use crate::{make_thing, ResourceEnum};
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsSchema {
	pub id: Thing,
	pub name: String,
	pub is_deleted: bool,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl Default for PermissionsSchema {
	fn default() -> Self {
		PermissionsSchema {
			id: make_thing(
				&ResourceEnum::Permissions.to_string(),
				&Uuid::new_v4().to_string(),
			),
			name: String::new(),
			is_deleted: false,
			created_at: None,
			updated_at: None,
		}
	}
}
