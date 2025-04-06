use super::AuthOtpSchema;
use crate::{make_thing, AppState, ResourceEnum, UsersItemDtoRaw};
use anyhow::{anyhow, bail, Result};
use chrono::{Duration, Utc};

pub struct AuthRepository<'a> {
	pub state: &'a AppState,
}

impl<'a> AuthRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	pub async fn query_store_user(&self, user: UsersItemDtoRaw) -> Result<String> {
		if user.email.trim().is_empty() {
			bail!("Email is required");
		}
		let table = ResourceEnum::UsersCache.to_string();
		let user_id = user.email.clone();
		let id = make_thing(&table, &user_id);
		let _ = self
			.state
			.surrealdb_mem
			.delete::<Option<UsersItemDtoRaw>>((table.clone(), user_id.clone()))
			.await?;
		let mut user_to_store = user.clone();
		user_to_store.id = id.clone();
		let record: Option<UsersItemDtoRaw> = self
			.state
			.surrealdb_mem
			.create((table, user_id))
			.content(user_to_store)
			.await?;
		match record {
			Some(_) => Ok("Success store user data".to_string()),
			None => bail!("Failed store user data"),
		}
	}

	pub async fn query_get_stored_user(
		&self,
		email: String,
	) -> Result<UsersItemDtoRaw> {
		let user: Option<UsersItemDtoRaw> = self
			.state
			.surrealdb_mem
			.select((ResourceEnum::UsersCache.to_string(), email))
			.await?;
		match user {
			Some(u) => Ok(u),
			None => bail!("No stored user data found"),
		}
	}

	pub async fn query_delete_stored_user(&self, email: String) -> Result<String> {
		let record: Option<UsersItemDtoRaw> = self
			.state
			.surrealdb_mem
			.delete((ResourceEnum::UsersCache.to_string(), email))
			.await?;
		dbg!(record.clone());
		match record {
			Some(_) => Ok("Success delete stored user".to_string()),
			None => bail!("Failed delete stored user"),
		}
	}

	pub async fn query_get_stored_otp(&self, email: String) -> Result<u32> {
		let table = ResourceEnum::OtpCache.to_string();
		let key = (table.as_str(), email.as_str());
		let result: Option<AuthOtpSchema> = self.state.surrealdb_mem.select(key).await?;
		match result {
			Some(data) => match Utc::now() > data.expires_at {
				true => {
					let _ = self
						.state
						.surrealdb_mem
						.delete::<Option<AuthOtpSchema>>(key)
						.await?;
					Err(anyhow!("OTP expired"))
				}
				false => Ok(data.otp),
			},
			None => bail!("No stored OTP found"),
		}
	}

	pub async fn query_store_otp(&self, email: String, otp: u32) -> Result<String> {
		let expires_at = Utc::now() + Duration::seconds(300);
		let table = ResourceEnum::OtpCache.to_string();
		let record: Option<AuthOtpSchema> = self
			.state
			.surrealdb_mem
			.create((table.as_str(), email.as_str()))
			.content(AuthOtpSchema { otp, expires_at })
			.await?;
		match record {
			Some(_) => Ok("Success store otp".to_string()),
			None => bail!("Failed store otp"),
		}
	}

	pub async fn query_delete_stored_otp(&self, email: String) -> Result<String> {
		let record: Option<AuthOtpSchema> = self
			.state
			.surrealdb_mem
			.delete((ResourceEnum::OtpCache.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete stored otp".to_string()),
			None => bail!("Failed delete stored otp"),
		}
	}
}
