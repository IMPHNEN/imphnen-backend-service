//! Service abstractions for the application
//! Provides traits and implementations for user lookup and authentication services
//! with PostgreSQL integration and comprehensive error handling

use crate::{postgres::PostgresError, AppState};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;
use imphnen_entities::seaorm::auth::roles::Entity as RolesEntity;
use imphnen_entities::seaorm::auth::users::Model as UserModel;
use imphnen_entities::UsersDetailQueryDto;
use imphnen_entities::PermissionsQueryDto;
use sea_orm::prelude::Json;
use sea_orm::{
	ActiveModelTrait,
	ActiveValue,
	ColumnTrait,
	EntityTrait,
	PaginatorTrait,
	QueryFilter,
    QuerySelect,
};
use std::result::Result;
use thiserror::Error;
use uuid::Uuid;

/// Service-related errors
#[derive(Debug, Error)]
pub enum ServiceError {
	#[error("User not found: {0}")]
	UserNotFound(String),

	#[error("Database error: {0}")]
	DatabaseError(#[from] sea_orm::DbErr),

	#[error("Connection error: {0}")]
	ConnectionError(#[from] PostgresError),

	#[error("Authentication failed: {0}")]
	AuthenticationFailed(String),

	#[error("Authorization failed: {0}")]
	AuthorizationFailed(String),

	#[error("Validation error: {0}")]
	ValidationError(String),

	#[error("Internal service error: {0}")]
	InternalError(String),
}

/// User reference types for different identification methods
#[derive(Debug, Clone)]
pub enum UserReference {
	/// User ID (UUID)
	Id(Uuid),
	/// User email address
	Email(String),
	/// User username
	Username(String),
	/// PostgreSQL-specific user model
	Model(UserModel),
}

/// Extended user information with additional computed fields
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtendedUserInfo {
	pub basic_info: UsersDetailQueryDto,
	pub last_login_at: Option<DateTime<Utc>>,
	pub login_count: u64,
	pub account_age_days: i64,
	pub is_recently_active: bool,
}

/// User registration data structure
#[derive(Debug, Clone)]
pub struct UserRegistrationData {
	pub id: Option<Uuid>,
	pub email: String,
	pub password_hash: String,
	pub username: String,
	pub first_name: Option<String>,
	pub last_name: Option<String>,
	pub avatar_url: Option<String>,
	pub metadata: Option<Json>,
	pub role_id: Option<Uuid>,
}

/// Convert UserModel to UsersDetailQueryDto
fn model_to_dto(model: &UserModel, role_model: Option<&imphnen_entities::seaorm::auth::roles::Model>) -> UsersDetailQueryDto {
	let mut dto = UsersDetailQueryDto::default();
	dto.id = model.id.to_string();
	dto.fullname = format!("{} {}", model.first_name.as_deref().unwrap_or(""), model.last_name.as_deref().unwrap_or("")).trim().to_string();
	dto.legal_name = None;
	dto.email = model.email.clone();
	dto.avatar = model.avatar_url.clone();
	dto.is_active = model.is_active;
	dto.is_deleted = model.deleted_at.is_some();
	dto.profile_extension = model.metadata.clone().and_then(|m| serde_json::from_value(m).ok());
	dto.password = String::new();
	
	if let Some(role) = role_model {
		let mut role_dto = imphnen_entities::RolesDetailQueryDto::default();
		role_dto.id = role.id.to_string();
		role_dto.name = role.name.clone();
		role_dto.is_deleted = false; 
		
		// Populate permissions
		if let Some(perms_json) = &role.permissions {
             println!("DEBUG: perms_json: {:?}", perms_json);
             if let Ok(perms_list) = serde_json::from_value::<Vec<String>>(perms_json.clone()) {
                 println!("DEBUG: perms_list: {:?}", perms_list);
                 let dtos = perms_list.into_iter().map(|p| {
                     // Create PermissionsQueryDto wrapped in Option
                     Some(PermissionsQueryDto {
                         id: Some(p.clone()),
                         name: Some(p),
                         created_at: None,
                         updated_at: None,
                     })
                 }).collect();
                 role_dto.permissions = Some(dtos);
             }
		}
		
		dto.role = role_dto;
	} else {
		dto.role = imphnen_entities::RolesDetailQueryDto::default();
	}

	dto.created_at = model.created_at.to_rfc3339();
	dto.updated_at = model.updated_at.to_rfc3339();
	dto.mentor_id = None;
	dto.from_profile_extension()
}

/// User lookup service trait with comprehensive user retrieval methods
#[async_trait]
pub trait UserLookupService: Send + Sync {
	async fn get_user_by_id(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError>;

	async fn get_user_by_email(
		&self,
		email: &str,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError>;

	async fn get_user_by_username(
		&self,
		username: &str,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError>;

	async fn get_user_by_reference(
		&self,
		reference: UserReference,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError>;

	async fn user_exists(
		&self,
		reference: UserReference,
		state: &AppState,
	) -> Result<bool, ServiceError>;

	async fn get_users_by_ids(
		&self,
		user_ids: Vec<Uuid>,
		state: &AppState,
	) -> Result<Vec<ExtendedUserInfo>, ServiceError>;

	async fn search_users(
		&self,
		query: &str,
		offset: u64,
		limit: u64,
		state: &AppState,
	) -> Result<Vec<ExtendedUserInfo>, ServiceError>;

	async fn count_users(&self, state: &AppState) -> Result<u64, ServiceError>;
}

/// Authentication repository trait with comprehensive auth operations
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

/// Default implementation of UserLookupService using PostgreSQL
pub struct PostgresUserLookupService;

impl Default for PostgresUserLookupService {
    fn default() -> Self {
        Self::new()
    }
}

impl PostgresUserLookupService {
	pub fn new() -> Self {
		Self
	}

	/// Convert UserModel to ExtendedUserInfo
	fn model_to_extended_info(&self, model: UserModel, role_model: Option<imphnen_entities::seaorm::auth::roles::Model>) -> ExtendedUserInfo {
		let basic_info = model_to_dto(&model, role_model.as_ref());

		let account_age_days = (Utc::now() - model.created_at).num_days();
		let is_recently_active =
			model.updated_at > Utc::now() - chrono::Duration::days(30);

		ExtendedUserInfo {
			basic_info,
			last_login_at: None,
			login_count: 0,
			account_age_days,
			is_recently_active,
		}
	}
}

#[async_trait]
impl UserLookupService for PostgresUserLookupService {
	async fn get_user_by_id(
		&self,
		user_id: Uuid,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let (user, role) = UsersEntity::find_by_id(user_id)
            .find_also_related(RolesEntity)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with ID {user_id} not found"))
			})?;

		Ok(self.model_to_extended_info(user, role))
	}

	async fn get_user_by_email(
		&self,
		email: &str,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let (user, role) = UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
            .find_also_related(RolesEntity)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!("User with email {email} not found"))
			})?;

		Ok(self.model_to_extended_info(user, role))
	}

	async fn get_user_by_username(
		&self,
		username: &str,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let (user, role) = UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Username.eq(username))
            .find_also_related(RolesEntity)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!(
					"User with username {} not found",
					username
				))
			})?;

		Ok(self.model_to_extended_info(user, role))
	}

	async fn get_user_by_reference(
		&self,
		reference: UserReference,
		state: &AppState,
	) -> Result<ExtendedUserInfo, ServiceError> {
		match reference {
			UserReference::Id(id) => self.get_user_by_id(id, state).await,
			UserReference::Email(email) => self.get_user_by_email(&email, state).await,
			UserReference::Username(username) => {
				self.get_user_by_username(&username, state).await
			}
			UserReference::Model(model) => {
                let role = if let Some(role_id) = model.role_id {
                     RolesEntity::find_by_id(role_id).one(&state.postgres_connection.conn).await.unwrap_or(None)
                } else {
                    None
                };
                Ok(self.model_to_extended_info(model, role))
            },
		}
	}

	async fn user_exists(
		&self,
		reference: UserReference,
		state: &AppState,
	) -> Result<bool, ServiceError> {
		let exists = match reference {
			UserReference::Id(id) => {
				use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

				UsersEntity::find_by_id(id)
					.count(&state.postgres_connection.conn)
					.await
					.map_err(ServiceError::DatabaseError)?
					> 0
			}
			UserReference::Email(email) => {
				use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

				UsersEntity::find()
					.filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(&email))
					.count(&state.postgres_connection.conn)
					.await
					.map_err(ServiceError::DatabaseError)?
					> 0
			}
			UserReference::Username(username) => {
				use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

				UsersEntity::find()
					.filter(
						imphnen_entities::seaorm::auth::users::Column::Username.eq(&username),
					)
					.count(&state.postgres_connection.conn)
					.await
					.map_err(ServiceError::DatabaseError)?
					> 0
			}
			UserReference::Model(_) => true,
		};

		Ok(exists)
	}

	async fn get_users_by_ids(
		&self,
		user_ids: Vec<Uuid>,
		state: &AppState,
	) -> Result<Vec<ExtendedUserInfo>, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let users_with_roles = UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Id.is_in(user_ids))
            .find_also_related(RolesEntity)
			.all(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;

		Ok(
			users_with_roles
				.into_iter()
				.map(|(user, role)| self.model_to_extended_info(user, role))
				.collect(),
		)
	}

	async fn search_users(
		&self,
		query: &str,
		offset: u64,
		limit: u64,
		state: &AppState,
	) -> Result<Vec<ExtendedUserInfo>, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let search_pattern = format!("%{query}%");

		let users_with_roles = UsersEntity::find()
			.filter(
				imphnen_entities::seaorm::auth::users::Column::Email
					.contains(&search_pattern)
					.or(
						imphnen_entities::seaorm::auth::users::Column::Username
							.contains(&search_pattern),
					)
					.or(
						imphnen_entities::seaorm::auth::users::Column::FirstName
							.contains(&search_pattern),
					)
					.or(
						imphnen_entities::seaorm::auth::users::Column::LastName
							.contains(&search_pattern),
					),
			)
			.offset(offset)
			.limit(limit)
            .find_also_related(RolesEntity)
			.all(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;

		Ok(
			users_with_roles
				.into_iter()
				.map(|(user, role)| self.model_to_extended_info(user, role))
				.collect(),
		)
	}

	async fn count_users(&self, state: &AppState) -> Result<u64, ServiceError> {
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

		let count = UsersEntity::find()
			.count(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;

		Ok(count)
	}
}

/// Default implementation of AuthRepositoryTrait using PostgreSQL
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
		use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

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
		use imphnen_entities::seaorm::auth::users::{
			ActiveModel, Entity as UsersEntity,
		};

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
		user_registration_data: UserRegistrationData,
		state: &AppState,
	) -> Result<UserModel, ServiceError> {
		use imphnen_entities::seaorm::auth::users::ActiveModel;
		let user_id = user_registration_data.id.unwrap_or_else(Uuid::new_v4); // Use provided ID or generate new
		let active_model = ActiveModel {
			id: ActiveValue::Set(user_id),
			email: ActiveValue::Set(user_registration_data.email),
			password_hash: ActiveValue::Set(user_registration_data.password_hash),
			username: ActiveValue::Set(user_registration_data.username),
			first_name: ActiveValue::Set(user_registration_data.first_name),
			last_name: ActiveValue::Set(user_registration_data.last_name),
			avatar_url: ActiveValue::Set(user_registration_data.avatar_url),
			is_verified: ActiveValue::Set(false),
			is_active: ActiveValue::Set(true),
			// Role-based permissions will determine admin access.
			metadata: ActiveValue::Set(user_registration_data.metadata),
			created_at: ActiveValue::Set(Utc::now()),
			updated_at: ActiveValue::Set(Utc::now()),
			deleted_at: ActiveValue::Set(None),
			role_id: ActiveValue::Set(user_registration_data.role_id),
		};

		let created_user: UserModel = active_model
			.insert(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;

		Ok(created_user)
	}
	async fn update_password(
		&self,
		user_id: Uuid,
		new_password_hash: &str,
		state: &AppState,
	) -> Result<(), ServiceError> {
		use imphnen_entities::seaorm::auth::users::{
			ActiveModel, Entity as UsersEntity,
		};

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
		use imphnen_entities::seaorm::auth::users::{
			ActiveModel, Entity as UsersEntity,
		};

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
		use imphnen_entities::seaorm::auth::users::{
			ActiveModel, Entity as UsersEntity,
		};

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

		// Determine permissions from role if available. Fall back to verification-based permissions.
		let permissions = if let Some(role_id) = user.role_id {
			// Try to fetch the role from DB and return its configured permissions
			match RolesEntity::find_by_id(role_id).one(&state.postgres_connection.conn).await.map_err(ServiceError::DatabaseError)? {
				Some(role) => {
                    let perms = if let Some(perms_json) = role.permissions.clone() {
						serde_json::from_value::<Vec<String>>(perms_json).unwrap_or_default()
					} else {
						vec![]
					};

					if role.is_system_role {
                        if perms.is_empty() {
						    vec!["admin.*".to_string(), "user.*".to_string(), "content.*".to_string()]
                        } else {
                            perms
                        }
					} else if perms.is_empty() {
     						    if user.is_verified {
     							    vec!["user.read".to_string(), "user.update".to_string(), "content.read".to_string()]
     						    } else {
     							    vec!["user.read".to_string(), "content.read".to_string()]
     						    }
                             } else {
                                 perms
                             }
				}
				None => {
					if user.is_verified {
						vec!["user.read".to_string(), "user.update".to_string(), "content.read".to_string()]
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_user_reference_creation() {
		let id_ref = UserReference::Id(Uuid::new_v4());
		let email_ref = UserReference::Email("test@example.com".to_string());
		let username_ref = UserReference::Username("testuser".to_string());

		assert!(matches!(id_ref, UserReference::Id(_)));
		assert!(matches!(email_ref, UserReference::Email(_)));
		assert!(matches!(username_ref, UserReference::Username(_)));
	}

	#[test]
	fn test_service_error_types() {
		let error = ServiceError::UserNotFound("Test user".to_string());
		assert_eq!(error.to_string(), "User not found: Test user");

		let error = ServiceError::AuthenticationFailed("Invalid password".to_string());
		assert_eq!(error.to_string(), "Authentication failed: Invalid password");
	}

	#[test]
	fn test_user_registration_data() {
		let registration_data = UserRegistrationData {
			id: None,
			email: "test@example.com".to_string(),
			password_hash: "hashed_password".to_string(),
			username: "testuser".to_string(),
			first_name: Some("Test".to_string()),
			last_name: Some("User".to_string()),
			avatar_url: None,
			metadata: None,
			role_id: None,
		};

		assert_eq!(registration_data.email, "test@example.com");
		assert_eq!(registration_data.username, "testuser");
	}
}
