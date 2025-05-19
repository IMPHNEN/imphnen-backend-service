use crate::{RolesDetailItemDto, RolesDetailQueryDto};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use utoipa::ToSchema;
use validator::Validate;

lazy_static! {
	static ref PASSWORD_REGEX: Regex = Regex::new(r"^[A-Za-z\d@$!%*?&]{8,}$").unwrap();
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersActiveInactiveRequestDto {
	pub is_active: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersSetNewPasswordRequestDto {
	pub password: String,
	pub old_password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersCreateRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,

	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(regex(
		path = "PASSWORD_REGEX",
		message = "Password must include uppercase, lowercase, number, and special character"
	))]
	pub password: String,

	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,

	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,
	pub is_active: bool,
	pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UsersUpdateRequestDto {
	#[validate(
		length(min = 1, message = "Email cannot be empty"),
		email(message = "Email not valid")
	)]
	pub email: String,
	#[validate(length(
		min = 8,
		message = "Password must have at least 8 characters"
	))]
	#[validate(length(min = 2, message = "Fullname at least have 2 character"))]
	pub fullname: String,
	#[validate(length(
		min = 10,
		message = "Phone number at least have 10 character"
	))]
	pub phone_number: String,
	pub is_active: bool,
	#[validate(length(min = 1, message = "Gender is required"))]
	pub gender: Option<String>,
	#[validate(length(min = 1, message = "Birthdate is required"))]
	pub birthdate: Option<String>,
	#[validate(length(min = 1, message = "Avatar is required"))]
	pub avatar: Option<String>,
	pub role_id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersDetailItemDto {
	pub id: String,
	pub role: RolesDetailItemDto,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersDetailItemDto {
	pub fn from(dto: &UsersDetailQueryDto) -> Self {
		Self {
			id: dto.id.id.to_raw().clone(),
			role: RolesDetailItemDto::from(&dto.role),
			fullname: dto.fullname.clone(),
			email: dto.email.clone(),
			avatar: dto.avatar.clone(),
			phone_number: dto.phone_number.clone(),
			is_active: dto.is_active.clone(),
			gender: dto.gender.clone(),
			birthdate: dto.birthdate.clone(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UsersListItemDto {
	pub id: String,
	pub role: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersListQueryDto {
	pub id: Thing,
	pub role: RolesDetailQueryDto,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

impl UsersListQueryDto {
	pub fn from(&self, role: String) -> UsersListItemDto {
		UsersListItemDto {
			id: self.id.id.to_raw(),
			role,
			fullname: self.fullname.clone(),
			email: self.email.clone(),
			avatar: self.avatar.clone(),
			phone_number: self.phone_number.clone(),
			is_active: self.is_active,
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UsersDetailQueryDto {
	pub id: Thing,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub phone_number: String,
	pub is_active: bool,
	pub is_deleted: bool,
	pub gender: Option<String>,
	pub birthdate: Option<String>,
	pub password: String,
	pub role: RolesDetailQueryDto,
	pub created_at: String,
	pub updated_at: String,
}
