use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;
use imphnen_entities::seaorm::auth::users::Model as UserModel;
use chrono::Utc;
use async_trait::async_trait;
use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait, ActiveModelTrait, ActiveValue};
use imphnen_libs::{AuthRepositoryTrait, services::ServiceError, services::UserRegistrationData, AppState, AppStatePostgresExt};
use uuid::Uuid;

/// PostgreSQL-based authentication repository
///
/// This repository handles authentication-related database operations
/// using SeaORM for PostgreSQL integration.
pub struct AuthRepository<'a> {
pub db: &'a DatabaseConnection,
}

impl<'a> AuthRepository<'a> {
pub fn new(state: &'a AppState) -> Self {
    Self { db: state.postgres_db() }
}
}

/// PostgreSQL-based implementation of authentication repository
///
/// This implementation completes the migration from SurrealDB to PostgreSQL using SeaORM.
/// All database operations now use native PostgreSQL queries through SeaORM's entity system.
#[async_trait]
impl AuthRepositoryTrait for AuthRepository<'_> {
    async fn get_user_for_auth(&self, email: &str, _state: &AppState) -> Result<UserModel, ServiceError> {
        UsersEntity::find()
            .filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::UserNotFound(format!("User with email {email} not found")))
    }

    async fn validate_credentials(&self, email: &str, password: &str, state: &AppState) -> Result<UserModel, ServiceError> {
        use imphnen_libs::argon::verify_password;

        let user = self.get_user_for_auth(email, state).await?;

        if !user.is_active {
            return Err(ServiceError::AuthenticationFailed("Account is deactivated".to_string()));
        }

        if !user.is_verified {
            return Err(ServiceError::AuthenticationFailed("Account not verified".to_string()));
        }

        let is_valid = verify_password(password, &user.password_hash)
            .map_err(|e| ServiceError::InternalError(format!("Password verification failed: {e}")))?;

        if !is_valid {
            return Err(ServiceError::AuthenticationFailed("Invalid password".to_string()));
        }

        Ok(user)
    }

    async fn update_last_login(&self, user_id: Uuid, _state: &AppState) -> Result<(), ServiceError> {
        let user = UsersEntity::find_by_id(user_id)
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::UserNotFound(format!("User with ID {} not found", user_id)))?;

        let mut active_model: imphnen_entities::seaorm::auth::users::ActiveModel = user.into();
        active_model.updated_at = ActiveValue::Set(Utc::now());

        active_model
            .update(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn create_user(&self, user_data: UserRegistrationData, _state: &AppState) -> Result<UserModel, ServiceError> {
        // Check if user already exists
        if UsersEntity::find()
            .filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(&user_data.email))
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .is_some() {
            return Err(ServiceError::ValidationError("User already exists".to_string()));
        }

        let first_name = user_data.first_name.unwrap_or_default();
        let last_name = user_data.last_name.unwrap_or_default();

        let active_model = imphnen_entities::seaorm::auth::users::ActiveModel {
            id: ActiveValue::Set(Uuid::new_v4()),
            email: ActiveValue::Set(user_data.email.clone()),
            password_hash: ActiveValue::Set(user_data.password_hash),
            username: ActiveValue::Set(user_data.email.clone()), // Use email as username for now
            first_name: ActiveValue::Set(Some(first_name.to_string())),
            last_name: ActiveValue::Set(Some(last_name.to_string())),
            avatar_url: ActiveValue::Set(user_data.avatar_url),
            is_verified: ActiveValue::Set(false),
            is_active: ActiveValue::Set(true),
            metadata: ActiveValue::Set(None),
            created_at: ActiveValue::Set(Utc::now()),
            updated_at: ActiveValue::Set(Utc::now()),
            deleted_at: ActiveValue::Set(None),
            role_id: ActiveValue::Set(user_data.role_id),
        };

        let user = UsersEntity::insert(active_model)
            .exec(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?;

        // Get the created user
        UsersEntity::find_by_id(user.last_insert_id)
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::InternalError("Failed to retrieve created user".to_string()))
    }

    async fn update_password(&self, user_id: Uuid, new_password_hash: &str, _state: &AppState) -> Result<(), ServiceError> {
        let user = UsersEntity::find_by_id(user_id)
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::UserNotFound(format!("User with ID {user_id} not found")))?;

        let mut active_model: imphnen_entities::seaorm::auth::users::ActiveModel = user.into();
        active_model.password_hash = ActiveValue::Set(new_password_hash.to_string());
        active_model.updated_at = ActiveValue::Set(Utc::now());

        active_model
            .update(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn deactivate_user(&self, user_id: Uuid, _state: &AppState) -> Result<(), ServiceError> {
        let user = UsersEntity::find_by_id(user_id)
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::UserNotFound(format!("User with ID {} not found", user_id)))?;

        let mut active_model: imphnen_entities::seaorm::auth::users::ActiveModel = user.into();
        active_model.is_active = ActiveValue::Set(false);
        active_model.updated_at = ActiveValue::Set(Utc::now());

        active_model
            .update(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn reactivate_user(&self, user_id: Uuid, _state: &AppState) -> Result<(), ServiceError> {
        let user = UsersEntity::find_by_id(user_id)
            .one(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?
            .ok_or_else(|| ServiceError::UserNotFound(format!("User with ID {} not found", user_id)))?;

        let mut active_model: imphnen_entities::seaorm::auth::users::ActiveModel = user.into();
        active_model.is_active = ActiveValue::Set(true);
        active_model.updated_at = ActiveValue::Set(Utc::now());

        active_model
            .update(self.db)
            .await
            .map_err(ServiceError::DatabaseError)?;

        Ok(())
    }

    async fn get_user_permissions(&self, _user_id: Uuid, _state: &AppState) -> Result<Vec<String>, ServiceError> {
        // This is a simplified implementation - in a real app you'd join with roles_permissions
        // For now, return empty vec
        Ok(vec![])
    }

    async fn has_permission(&self, _user_id: Uuid, _permission: &str, _state: &AppState) -> Result<bool, ServiceError> {
        // This is a simplified implementation - in a real app you'd check roles_permissions
        // For now, return false
        Ok(false)
    }
}

// Re-export the Postgres-backed auth repository implementation from imphnen-libs
// There is an existing generic implementation in imphnen-libs::services::PostgresAuthRepository
// Re-export it here so other crates can import a stable name `AuthRepoImpl` as expected.
pub use imphnen_libs::services::PostgresAuthRepository as AuthRepoImpl;
