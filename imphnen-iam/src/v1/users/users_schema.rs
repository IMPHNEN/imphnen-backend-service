use super::{UsersDetailItemDto, UsersListItemDto};
use crate::RolesItemDto;
use imphnen_utils::Crud;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

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

impl Crud<UsersListItemDto, String> for UsersSchema {
	fn list(&self, role: String) -> UsersListItemDto {
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

impl Crud<UsersDetailItemDto, RolesItemDto> for UsersSchema {
	fn detail(&self, role: RolesItemDto) -> UsersDetailItemDto {
		UsersDetailItemDto {
			id: self.id.id.to_raw(),
			role,
			fullname: self.fullname.clone(),
			email: self.email.clone(),
			avatar: self.avatar.clone(),
			phone_number: self.phone_number.clone(),
			is_active: self.is_active,
			gender: self.gender.clone(),
			birthdate: self.birthdate.clone(),
			created_at: self.created_at.clone(),
			updated_at: self.updated_at.clone(),
		}
	}
}

impl UsersSchema {
	pub fn list_from(&self, role: String) -> UsersListItemDto {
		self.list(role)
	}

	pub fn detail_from(&self, role: RolesItemDto) -> UsersDetailItemDto {
		self.detail(role)
	}
}
