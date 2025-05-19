use super::{UsersCreateRequestDto, UsersDetailQueryDto, UsersUpdateRequestDto};
use imphnen_libs::{ResourceEnum, hash_password};
use imphnen_utils::{get_iso_date, make_thing};
use serde::{Deserialize, Serialize};
use surrealdb::{Uuid, sql::Thing};

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
		Self {
			id: Thing::from(("app_users", "dummy")),
			fullname: "".into(),
			email: "".into(),
			password: "".into(),
			avatar: None,
			phone_number: "".into(),
			is_active: false,
			is_deleted: false,
			gender: None,
			birthdate: None,
			role: Thing::from(("app_roles", "dummy")),
			created_at: "".into(),
			updated_at: "".into(),
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
			gender: dto.gender,
			birthdate: dto.birthdate,
			password: dto.password,
			created_at: dto.created_at,
			updated_at: dto.updated_at,
			role: make_thing(&ResourceEnum::Roles.to_string(), &dto.role.id.to_string()),
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
			gender: None,
			birthdate: None,
			avatar: None,
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
}
