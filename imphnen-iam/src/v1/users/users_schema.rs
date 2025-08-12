use super::{UsersCreateRequestDto, UsersDetailQueryDto, UsersUpdateRequestDto};
use imphnen_libs::{ResourceEnum, hash_password};
use imphnen_utils::extract_id;
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersSchema {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub password: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub is_deleted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentor_id: Option<Thing>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthdate: Option<String>,
	pub role: Thing,
	pub created_at: String,
	pub updated_at: String,
}

impl Default for UsersSchema {
	fn default() -> Self {
		Self {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: String::new(),
			email: String::new(),
			password: hash_password("").unwrap(),
			avatar: None,
			phone_number: String::new(),
			is_active: false,
			is_deleted: false,
			mentor_id: Some(make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			)),
			gender: None,
			birthdate: None,
			role: make_thing(
				&ResourceEnum::Roles.to_string(),
				"5713cb37-dc02-4e87-8048-d7a41d352059",
			),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}
}

impl UsersSchema {
	pub fn from(dto: UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id,
			fullname: dto.fullname,
			email: dto.email,
			avatar: dto.avatar,
			phone_number: dto.phone_number,
			is_active: dto.is_active,
			is_deleted: dto.is_deleted,
			mentor_id: Some(dto.mentor_id.unwrap_or_else(|| {
				make_thing(
					&ResourceEnum::Users.to_string(),
					&Uuid::new_v4().to_string(),
				)
			})),
			gender: dto.gender,
			birthdate: dto.birthdate,
			password: dto.password,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			role: make_thing(&ResourceEnum::Roles.to_string(), &extract_id(&dto.role.id)),
		}
	}

	pub fn update(user: UsersUpdateRequestDto, id: String) -> Self {
		Self {
			id: make_thing(&ResourceEnum::Users.to_string(), &id),
			fullname: user.fullname,
			email: user.email,
			phone_number: user.phone_number,
			is_active: user.is_active,
			gender: user.gender,
			birthdate: user.birthdate,
			avatar: user.avatar,
			is_deleted: false,
			role: make_thing(&ResourceEnum::Roles.to_string(), &user.role_id),
			updated_at: get_iso_date(),
			..Default::default()
		}
	}

	pub fn create(user: UsersCreateRequestDto) -> Self {
		let password = hash_password(&user.password).unwrap();
		Self {
			id: make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			),
			fullname: user.fullname,
			email: user.email,
			password,
			phone_number: user.phone_number,
			is_active: false,
			mentor_id: Some(make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			)),
			gender: None,
			birthdate: None,
			avatar: user.avatar,
			is_deleted: false,
			role: make_thing(&ResourceEnum::Roles.to_string(), &user.role_id),
			created_at: get_iso_date(),
			updated_at: get_iso_date(),
		}
	}

	pub fn patch_password(dto: UsersDetailQueryDto, password: String) -> Self {
		Self {
			password,
			id: dto.id.clone(),
			..Self::from(dto)
		}
	}

	pub fn update_mentor_id(mut self, mentor_id: Option<String>) -> Self {
		self.mentor_id = match mentor_id {
			Some(id) => Some(make_thing(&ResourceEnum::Users.to_string(), &id)),
			None => Some(make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			)),
		};
		self.updated_at = get_iso_date();
		self
	}
}
