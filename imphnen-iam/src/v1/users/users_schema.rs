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
	#[serde(skip_serializing_if = "Option::is_none")]
	pub legal_name: Option<String>,
	pub email: String,
	pub password: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub avatar: Option<String>,
	pub phone_number: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub phone_for_verification: Option<String>,
	pub is_active: bool,
	pub is_deleted: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mentor_id: Option<Thing>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub gender: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub birthdate: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub domicile: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identity_document_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bio: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_education: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub linkedin_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub github_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cv_url: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub portfolio_url: Option<String>,
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
			legal_name: None,
			email: String::new(),
			password: hash_password("").unwrap(),
			avatar: None,
			phone_number: String::new(),
			phone_for_verification: None,
			is_active: false,
			is_deleted: false,
			mentor_id: Some(make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			)),
			gender: None,
			birthdate: None,
			domicile: None,
			identity_document_url: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
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
			legal_name: dto.legal_name,
			email: dto.email,
			avatar: dto.avatar,
			phone_number: dto.phone_number,
			phone_for_verification: dto.phone_for_verification,
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
			domicile: dto.domicile,
			identity_document_url: dto.identity_document_url,
			bio: dto.bio,
			last_education: dto.last_education,
			linkedin_url: dto.linkedin_url,
			github_url: dto.github_url,
			cv_url: dto.cv_url,
			portfolio_url: dto.portfolio_url,
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
			legal_name: user.legal_name,
			email: user.email,
			phone_number: user.phone_number,
			phone_for_verification: user.phone_for_verification,
			is_active: user.is_active,
			gender: user.gender,
			birthdate: user.birthdate,
			domicile: user.domicile,
			identity_document_url: user.identity_document_url,
			bio: user.bio,
			last_education: user.last_education,
			linkedin_url: user.linkedin_url,
			github_url: user.github_url,
			cv_url: user.cv_url,
			portfolio_url: user.portfolio_url,
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
			legal_name: None,
			email: user.email,
			password,
			phone_number: user.phone_number,
			phone_for_verification: None,
			is_active: false,
			mentor_id: Some(make_thing(
				&ResourceEnum::Users.to_string(),
				&Uuid::new_v4().to_string(),
			)),
			gender: None,
			birthdate: None,
			domicile: None,
			identity_document_url: None,
			bio: None,
			last_education: None,
			linkedin_url: None,
			github_url: None,
			cv_url: None,
			portfolio_url: None,
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
