use async_trait::async_trait;
use imphnen_entities::seaorm::auth::roles::Entity as RolesEntity;
use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QuerySelect};
use std::result::Result;
use uuid::Uuid;

use super::dto::{ExtendedUserInfo, UserReference, model_to_dto};
use super::error::ServiceError;
use crate::AppState;

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

	fn model_to_extended_info(
		&self,
		model: imphnen_entities::seaorm::auth::users::Model,
		role_model: Option<imphnen_entities::seaorm::auth::roles::Model>,
	) -> ExtendedUserInfo {
		let basic_info = model_to_dto(&model, role_model.as_ref());
		let account_age_days = (chrono::Utc::now() - model.created_at).num_days();
		let is_recently_active =
			model.updated_at > chrono::Utc::now() - chrono::Duration::days(30);
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
		let (user, role) = UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Username.eq(username))
			.find_also_related(RolesEntity)
			.one(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?
			.ok_or_else(|| {
				ServiceError::UserNotFound(format!(
					"User with username {username} not found"
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
					RolesEntity::find_by_id(role_id)
						.one(&state.postgres_connection.conn)
						.await
						.unwrap_or(None)
				} else {
					None
				};
				Ok(self.model_to_extended_info(model, role))
			}
		}
	}

	async fn user_exists(
		&self,
		reference: UserReference,
		state: &AppState,
	) -> Result<bool, ServiceError> {
		let exists = match reference {
			UserReference::Id(id) => {
				UsersEntity::find_by_id(id)
					.count(&state.postgres_connection.conn)
					.await
					.map_err(ServiceError::DatabaseError)?
					> 0
			}
			UserReference::Email(email) => {
				UsersEntity::find()
					.filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(&email))
					.count(&state.postgres_connection.conn)
					.await
					.map_err(ServiceError::DatabaseError)?
					> 0
			}
			UserReference::Username(username) => {
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
		let users_with_roles = UsersEntity::find()
			.filter(imphnen_entities::seaorm::auth::users::Column::Id.is_in(user_ids))
			.find_also_related(RolesEntity)
			.all(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)?;
		Ok(
			users_with_roles
				.into_iter()
				.map(|(u, r)| self.model_to_extended_info(u, r))
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
		use imphnen_entities::seaorm::auth::users::Column;
		let pattern = format!("%{query}%");
		let users_with_roles = UsersEntity::find()
			.filter(
				Column::Email
					.contains(&pattern)
					.or(Column::Username.contains(&pattern))
					.or(Column::FirstName.contains(&pattern))
					.or(Column::LastName.contains(&pattern)),
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
				.map(|(u, r)| self.model_to_extended_info(u, r))
				.collect(),
		)
	}

	async fn count_users(&self, state: &AppState) -> Result<u64, ServiceError> {
		UsersEntity::find()
			.count(&state.postgres_connection.conn)
			.await
			.map_err(ServiceError::DatabaseError)
	}
}
