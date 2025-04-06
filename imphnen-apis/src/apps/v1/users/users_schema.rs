use crate::{get_iso_date, make_thing, ResourceEnum};
use serde::{Deserialize, Serialize};
use surrealdb::{sql::Thing, Uuid};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub password: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		UsersSchema {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: String::new(),
			email: String::new(),
			password: String::new(),
			avatar: None,
			phone_number: String::new(),
			is_active: false,
			is_deleted: false,
			gender: None,
			birthdate: None,
			role: make_thing(
				&ResourceEnum::Roles.to_string(),
				&Uuid::new_v4().to_string(),
			),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSetNewPasswordSchema {
	pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersActiveInactiveSchema {
	pub is_active: bool,
}
