use async_trait::async_trait;
use chrono::Utc;
use imphnen_entities::seaorm::auth::roles::Entity as RolesEntity;
use imphnen_entities::seaorm::auth::users::{
	Entity as UsersEntity, Model as UserModel,
};
use sea_orm::{
	ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter,
};
use std::result::Result;
use uuid::Uuid;

use super::dto::UserRegistrationData;
use super::error::ServiceError;
use crate::AppState;

#[async_trait]
pub trait AuthRepositoryTrait: Send + Sync {
	async fn get_user_for_auth(
		&self,
		email: &str,
		state: &AppState,
	) -> Result<UserModel, ServiceError>;
	async fn validate_credentials(
		&self,
		email: &str,
		password: &str,
		state: &AppState,
	) -> Result<UserModel, ServiceError>;
	async fn update_last_login(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError>;
	async fn create_user(
		&self,
		user_data: UserRegistrationData,
		state: &AppState,
	) -> Result<UserModel, ServiceError>;
	async fn update_password(
		&self,
		user_id: Uuid,
		new_password_hash: &str,
		state: &AppState,
	) -> Result<(), ServiceError>;
	async fn deactivate_user(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError>;
	async fn reactivate_user(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError>;
	async fn get_user_permissions(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<Vec<String>, ServiceError>;
	async fn has_permission(
		&self,
		user_id: Uuid,
		permission: &str,
		state: &AppState,
	) -> Result<bool, ServiceError>;
}

pub struct PostgresAuthRepository;

impl Default for PostgresAuthRepository {
	fn default() -> Self {
		Self::new()
	}
}

impl PostgresAuthRepository {
	pub fn new() -> Self {
		Self
	}
}

#[async_trait]
impl AuthRepositoryTrait for PostgresAuthRepository {
	async fn get_user_for_auth(
		&self,
		email: &str,
		state: &AppState,
	) -> Result<UserModel, ServiceError> {
		UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with email {email} not found"))
			})
	}

	async fn validate_credentials(
		&self,
		email: &str,
		password: &str,
		state: &AppState,
	) -> Result<UserModel, ServiceError> {
		use crate::argon::verify_password;
		let user = self.get_user_for_auth(email, state).await?;
		if !user.is_active {
			return Err(ServiceError::AuthenticationFailed(
				"Account is deactivated".to_string(),
			));
		}
		if !user.is_verified {
			return Err(ServiceError::AuthenticationFailed(
				"Account not verified".to_string(),
			));
		}
		let is_valid = verify_password(password, &user.password_hash).map_err(|e| {
			ServiceError::InternalError(format!("Password verification failed: {e}"))
		})?;
		if !is_valid {
			return Err(ServiceError::AuthenticationFailed(
				"Invalid password".to_string(),
			));
		}
		Ok(user)
	}

	async fn update_last_login(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user = UsersEntity::find_by_id(user_id)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;
		let mut active_model: ActiveModel = user.into();
		active_model.updated_at = ActiveValue::Set(Utc::now());
		active_model
			.update(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;
		Ok(())
	}

	async fn create_user(
		&self,
		data: UserRegistrationData,
		state: &AppState,
	) -> Result<UserModel, ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user_id = data.id.unwrap_or_else(Uuid::new_v4);
		let active_model = ActiveModel {
			id: ActiveValue::Set(user_id),
			email: ActiveValue::Set(data.email),
			password_hash: ActiveValue::Set(data.password_hash),
			username: ActiveValue::Set(data.username),
			first_name: ActiveValue::Set(data.first_name),
			last_name: ActiveValue::Set(data.last_name),
			avatar_url: ActiveValue::Set(data.avatar_url),
			is_verified: ActiveValue::Set(false),
			is_active: ActiveValue::Set(true),
			metadata: ActiveValue::Set(data.metadata),
			created_at: ActiveValue::Set(Utc::now()),
			updated_at: ActiveValue::Set(Utc::now()),
			deleted_at: ActiveValue::Set(None),
			role_id: ActiveValue::Set(data.role_id),
		};
		active_model
			.insert(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)
	}

	async fn update_password(
		&self,
		user_id: Uuid,
		new_password_hash: &str,
		state: &AppState,
	) -> Result<(), ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user = UsersEntity::find_by_id(user_id)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;
		let mut active_model: ActiveModel = user.into();
		active_model.password_hash = ActiveValue::Set(new_password_hash.to_string());
		active_model.updated_at = ActiveValue::Set(Utc::now());
		active_model
			.update(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;
		Ok(())
	}

	async fn deactivate_user(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user = UsersEntity::find_by_id(user_id)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;
		let mut active_model: ActiveModel = user.into();
		active_model.is_active = ActiveValue::Set(false);
		active_model.updated_at = ActiveValue::Set(Utc::now());
		active_model
			.update(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;
		Ok(())
	}

	async fn reactivate_user(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<(), ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user = UsersEntity::find_by_id(user_id)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;
		let mut active_model: ActiveModel = user.into();
		active_model.is_active = ActiveValue::Set(true);
		active_model.updated_at = ActiveValue::Set(Utc::now());
		active_model
			.update(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;
		Ok(())
	}

	async fn get_user_permissions(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<Vec<String>, ServiceError> {
		let user = UsersEntity::find_by_id(user_id)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;

		let permissions = if let Some(role_id) = user.role_id {
			match RolesEntity::find_by_id(role_id)
				.one(&state.postgres_connection.conn)
				.await
				.map_err(ServiceError::DatabaseError)?
			{
				Some(role) => {
					let perms = role
						.permissions
						.clone()
						.and_then(|j| serde_json::from_value::<Vec<String>>(j).ok())
						.unwrap_or_default();
					if role.is_system_role {
						if perms.is_empty() {
							vec![
								"admin.*".to_string(),
								"user.*".to_string(),
								"content.*".to_string(),
							]
						} else {
							perms
						}
					} else if perms.is_empty() {
						if user.is_verified {
							vec![
								"user.read".to_string(),
								"user.update".to_string(),
								"content.read".to_string(),
							]
						} else {
							vec!["user.read".to_string(), "content.read".to_string()]
						}
					} else {
						perms
					}
				}
				None => {
					if user.is_verified {
						vec![
							"user.read".to_string(),
							"user.update".to_string(),
							"content.read".to_string(),
						]
					} else {
						vec!["user.read".to_string(), "content.read".to_string()]
					}
				}
			}
		} else if user.is_verified {
			vec![
				"user.read".to_string(),
				"user.update".to_string(),
				"content.read".to_string(),
			]
		} else {
			vec!["user.read".to_string(), "content.read".to_string()]
		};

		Ok(permissions)
	}

	async fn has_permission(
		&self,
		user_id: Uuid,
		permission: &str,
		state: &AppState,
	) -> Result<bool, ServiceError> {
		let permissions = self.get_user_permissions(user_id, state).await?;
		Ok(
			permissions.contains(&permission.to_string())
				|| permissions.iter().any(|p| p.ends_with(".*")),
		)
	}
}
