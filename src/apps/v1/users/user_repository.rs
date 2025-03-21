use crate::{
	v1::{users_schema::UsersSchema, UsersItemDto},
	AppState, ResourceEnum,
};
use anyhow::{bail, Result};

use super::{CreateUserRequestDto, UpdateRequest, UpdateRequestDto};

pub struct UserRepository<'a> {
	state: &'a AppState,
}

impl<'a> UserRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_all_user(&self) -> Result<Vec<UsersSchema>> {
		let db = &self.state.surrealdb;
		Ok(db.select(ResourceEnum::Users.to_string()).await?)
	}

	pub async fn query_user_by_email(&self, email: &str) -> Result<UsersSchema> {
		let db = &self.state.surrealdb;
		let result = db
			.select((ResourceEnum::Users.to_string(), email))
			.await?;
		match result {
			Some(response) => Ok(response),
			None => {
				bail!("User not found")
			}
		}
	}

	pub async fn query_create_user(
		&self,
		data: CreateUserRequestDto,
	) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersItemDto> = db
			.create((ResourceEnum::Users.to_string(), &data.email))
			.content(UsersSchema {
				fullname: data.fullname.clone(),
				email: data.email.clone(),
				password: data.password.clone(),
				is_active: false,
			})
			.await?;
		match record {
			Some(_) => Ok("Success create user".into()),
			None => bail!("Failed to create user"),
		}
	}

	pub async fn query_update_user(&self, data: UpdateRequestDto) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersItemDto> = db
			.update((ResourceEnum::Users.to_string(), &data.email))
			.content(UpdateRequest {
				fullname: data.fullname.clone(),
				email: data.email.clone(),
			})
			.await?;
		match record {
			Some(_) => Ok("Success update user".into()),
			None => bail!("Failed to update user"),
		}
	}

	pub async fn query_delete_user(&self, email: &str) -> Result<String> {
		let db = &self.state.surrealdb;
		let record: Option<UsersItemDto> = db
			.delete((ResourceEnum::Users.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete user".into()),
			None => bail!("Failed to delete user"),
		}
	}
}
