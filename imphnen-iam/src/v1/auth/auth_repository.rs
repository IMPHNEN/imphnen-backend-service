use super::AuthOtpSchema;
use super::UserCacheSchema;
use imphnen_entities::{PermissionsQueryDto, RolesDetailQueryDto, UsersDetailQueryDto};
use crate::ResourceEnum;
use anyhow::{Result, anyhow, bail};
use chrono::{Duration, Utc};
use surrealdb::sql::Thing;
use tracing::instrument;
use tracing::info;
use async_trait::async_trait;
use imphnen_libs::AuthRepositoryTrait;
use imphnen_libs::SurrealMemClient;


pub struct AuthRepository {
	pub db: SurrealMemClient,
}

impl AuthRepository {
	pub fn new(db: SurrealMemClient) -> Self {
		Self { db }
	}

	#[instrument(skip(self, user), err)]
	pub async fn query_store_user(&self, user: UsersDetailQueryDto) -> Result<String> {
		if user.email.trim().is_empty() {
			bail!("Email is required");
		}
		let table = ResourceEnum::UsersCache.to_string();
		let user_id = user.email.clone();
		let permissions: Vec<String> =
			user.role.permissions.as_ref().unwrap_or(&vec![]).iter().filter_map(|p| p.as_ref().and_then(|pp| pp.name.clone())).collect();
		let user_cache = UserCacheSchema {
			email: user_id.clone(),
			permissions,
		};

		info!(query = %format!("DELETE FROM {} WHERE id = '{}'", table, user_id), "Executing SurrealDB query");
		let _record: Option<UserCacheSchema> = self
			.db
			.delete::<Option<UserCacheSchema>>((table.clone(), user_id.clone()))
			.await?;

		info!(query = %format!("CREATE {}:{}", table, user_id), "Executing SurrealDB query");
		let record: Option<UserCacheSchema> = self
			.db
			.create((table, user_id))
			.content(user_cache)
			.await?;

		match record {
			Some(_) => Ok("Success store user data".to_string()),
			None => bail!("Failed store user data"),
		}
	}

	#[instrument(skip(self, email), err)]
	pub async fn query_get_stored_user(
		&self,
		email: String,
	) -> Result<UsersDetailQueryDto> {
		info!(query = %format!("SELECT FROM {} WHERE id = '{}'", ResourceEnum::UsersCache.to_string(), email), "Executing SurrealDB query");
		let user_cache: Option<UserCacheSchema> = self
			.db
			.select((ResourceEnum::UsersCache.to_string(), email.clone()))
			.await?;

		match user_cache {
			Some(cache) => {
				let permissions_query_dto: Vec<PermissionsQueryDto> = cache
					.permissions
					.into_iter()
					.map(|name| PermissionsQueryDto {
						id: Some(Thing::from((
							"app_permissions".to_string(),
							surrealdb::sql::Id::rand(),
						))),
						name: Some(name),
						created_at: None,
						updated_at: None,
					})
					.collect();

				let role_detail_query_dto = RolesDetailQueryDto {
					id: Thing::from(("app_roles".to_string(), surrealdb::sql::Id::rand())),
					name: "CachedRole".to_string(),
					permissions: Some(permissions_query_dto.into_iter().map(Some).collect()),
					is_deleted: false,
					created_at: None,
					updated_at: None,
				};

				Ok(UsersDetailQueryDto {
					id: Thing::from(("app_users".to_string(), email.clone())),
					fullname: "Cached User".to_string(),
					legal_name: None,
					email: cache.email,
					avatar: None,
					phone_number: String::new(),
					phone_for_verification: None,
					is_active: true,
					is_deleted: false,
					gender: None,
					birthdate: None,
					domicile: None,
					bio: None,
					last_education: None,
					linkedin_url: None,
					github_url: None,
					cv_url: None,
					portfolio_url: None,
					website_url: None,
					twitter_url: None,
					location: None,
					skills: None,
					experience: None,
					education: None,
					career_status: None,
					password: String::new(),
					role: role_detail_query_dto,
					created_at: String::new(),
					updated_at: String::new(),
					mentor_id: None,
				})
			}
			None => bail!("No stored user data found"),
		}
	}

	#[instrument(skip(self, email), err)]
	pub async fn query_delete_stored_user(&self, email: String) -> Result<String> {
		info!(query = %format!("DELETE FROM {} WHERE id = '{}'", ResourceEnum::UsersCache.to_string(), email), "Executing SurrealDB query");
		let record: Option<UsersDetailQueryDto> = self
			.db
			.delete((ResourceEnum::UsersCache.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete stored user".to_string()),
			None => bail!("Failed delete stored user"),
		}
	}

	#[instrument(skip(self, email), err)]
	pub async fn query_get_stored_otp(&self, email: String) -> Result<u32> {
		let table = ResourceEnum::OtpCache.to_string();
		let key = (table.as_str(), email.as_str());
		info!(query = %format!("SELECT FROM {} WHERE id = '{}'", table, email), "Executing SurrealDB query");
		let result: Option<AuthOtpSchema> = self.db.select(key).await?;
		match result {
			Some(data) => match Utc::now() > data.expires_at {
				true => {
					info!(query = %format!("DELETE FROM {} WHERE id = '{}'", table, email), "Executing SurrealDB query");
					let _ = self
						.db
						.delete::<Option<AuthOtpSchema>>(key)
						.await?;
					Err(anyhow!("OTP expired"))
				}
				false => Ok(data.otp),
			},
			None => bail!("No stored OTP found"),
		}
	}

	#[instrument(skip(self, email, otp), err)]
	pub async fn query_store_otp(&self, email: String, otp: u32) -> Result<String> {
		let expires_at = Utc::now() + Duration::seconds(300);
		let table: String = ResourceEnum::OtpCache.to_string();
		info!(query = %format!("CREATE {}:{}", table, email), "Executing SurrealDB query");
		let record: Option<AuthOtpSchema> = self
			.db
			.create((table.as_str(), email.as_str()))
			.content(AuthOtpSchema { otp, expires_at })
			.await?;
		match record {
			Some(_) => Ok("Success store otp".to_string()),
			None => bail!("Failed store otp"),
		}
	}

	#[instrument(skip(self, email), err)]
	pub async fn query_delete_stored_otp(&self, email: String) -> Result<String> {
		info!(query = %format!("DELETE FROM {} WHERE id = '{}'", ResourceEnum::OtpCache.to_string(), email), "Executing SurrealDB query");
		let record: Option<AuthOtpSchema> = self
			.db
			.delete((ResourceEnum::OtpCache.to_string(), email))
			.await?;
		match record {
			Some(_) => Ok("Success delete stored otp".to_string()),
			None => bail!("Failed delete stored otp"),
		}
	}
}

pub struct AuthRepoImpl {
    pub db: SurrealMemClient,
}

#[async_trait]
impl AuthRepositoryTrait for AuthRepoImpl {
    async fn query_get_stored_user(
        &self,
        email: String,
    ) -> Result<UsersDetailQueryDto, anyhow::Error> {
        let repo = AuthRepository { db: self.db.clone() };
        repo.query_get_stored_user(email).await.map_err(|e| anyhow::anyhow!(e))
    }
}

