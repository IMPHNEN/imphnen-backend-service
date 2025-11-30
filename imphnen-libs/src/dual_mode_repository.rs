//! Dual-mode repository pattern implementation
//! Provides both PostgreSQL and in-memory repository implementations
//! with seamless switching between modes for testing and production

use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;
use sea_orm::{DbErr, ActiveModelTrait, ModelTrait, EntityTrait, QueryFilter, ColumnTrait, PaginatorTrait, QuerySelect, ActiveValue::Set,};

use crate::postgres::{PostgresConnection, PostgresError};
use imphnen_entities::seaorm::auth::users::{Entity as UsersEntity, Model as UserModel};
use imphnen_entities::seaorm::auth::roles::{Entity as RolesEntity, Model as RoleModel};

/// Error types for dual-mode repository operations
#[derive(Debug, Error)]
pub enum PostgresRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] DbErr),
    
    #[error("PostgreSQL connection error: {0}")]
    ConnectionError(#[from] PostgresError),
    
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Conversion error: {0}")]
    ConversionError(String),
    
    #[error("Repository operation failed: {0}")]
    OperationFailed(String),
}

/// Repository trait for user operations
#[async_trait]
pub trait PostgresRepository {
    /// Find user by ID
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, PostgresRepositoryError>;
    
    /// Find user by email
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserModel>, PostgresRepositoryError>;
    
    /// Find user by username
    async fn find_user_by_username(&self, username: &str) -> Result<Option<UserModel>, PostgresRepositoryError>;
    
    /// Create new user
    async fn create_user(&self, user: UserModel) -> Result<UserModel, PostgresRepositoryError>;
    
    /// Update user
    async fn update_user(&self, id: Uuid, user: UserModel) -> Result<UserModel, PostgresRepositoryError>;
    
    /// Delete user
    async fn delete_user(&self, id: Uuid) -> Result<(), PostgresRepositoryError>;
    
    /// List all users with pagination
    async fn list_users(&self, offset: u64, limit: u64) -> Result<Vec<UserModel>, PostgresRepositoryError>;
    
    /// Count total users
    async fn count_users(&self) -> Result<u64, PostgresRepositoryError>;
    
    /// Find role by ID
    async fn find_role_by_id(&self, id: Uuid) -> Result<Option<RoleModel>, PostgresRepositoryError>;
    
    /// Find role by name
    async fn find_role_by_name(&self, name: &str) -> Result<Option<RoleModel>, PostgresRepositoryError>;
    
    /// Create new role
    async fn create_role(&self, role: RoleModel) -> Result<RoleModel, PostgresRepositoryError>;
    
    /// Update role
    async fn update_role(&self, id: Uuid, role: RoleModel) -> Result<RoleModel, PostgresRepositoryError>;
    
    /// Delete role
    async fn delete_role(&self, id: Uuid) -> Result<(), PostgresRepositoryError>;
    
    /// List all roles
    async fn list_roles(&self) -> Result<Vec<RoleModel>, PostgresRepositoryError>;
}

/// Default PostgreSQL repository implementation
pub struct PostgresRepositoryDefaultImpl {
    connection: Arc<PostgresConnection>,
}

impl PostgresRepositoryDefaultImpl {
    /// Create a new PostgreSQL repository instance
    pub fn new(connection: Arc<PostgresConnection>) -> Self {
        Self { connection }
    }
    
    /// Get the underlying PostgreSQL connection
    pub fn connection(&self) -> &Arc<PostgresConnection> {
        &self.connection
    }
}

#[async_trait]
impl PostgresRepository for PostgresRepositoryDefaultImpl {
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<UserModel>, PostgresRepositoryError> {
        let user = UsersEntity::find_by_id(id)
            .one(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(user)
    }
    
    async fn find_user_by_email(&self, email: &str) -> Result<Option<UserModel>, PostgresRepositoryError> {
        let user = UsersEntity::find()
            .filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
            .one(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(user)
    }
    
    async fn find_user_by_username(&self, username: &str) -> Result<Option<UserModel>, PostgresRepositoryError> {
        let user = UsersEntity::find()
            .filter(imphnen_entities::seaorm::auth::users::Column::Username.eq(username))
            .one(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(user)
    }
    
    async fn create_user(&self, user: UserModel) -> Result<UserModel, PostgresRepositoryError> {
        let active_model = imphnen_entities::seaorm::auth::users::ActiveModel {
            id: Set(user.id),
            email: Set(user.email),
            password_hash: Set(user.password_hash),
            username: Set(user.username),
            first_name: Set(user.first_name),
            last_name: Set(user.last_name),
            avatar_url: Set(user.avatar_url),
            is_verified: Set(user.is_verified),
            is_active: Set(user.is_active),
            metadata: Set(user.metadata),
            created_at: Set(user.created_at),
            updated_at: Set(user.updated_at),
            deleted_at: Set(user.deleted_at), // Use user.deleted_at
            role_id: Set(user.role_id),
        };

        let created_user = active_model.insert(&self.connection.conn).await.map_err(PostgresRepositoryError::DatabaseError)?;
        Ok(created_user)
    }
    
    async fn update_user(&self, id: Uuid, user: UserModel) -> Result<UserModel, PostgresRepositoryError> {
        let existing_user = self.find_user_by_id(id).await?
            .ok_or_else(|| PostgresRepositoryError::NotFound(format!("User with id {id} not found")))?;
        
        let mut active_model: imphnen_entities::seaorm::auth::users::ActiveModel = existing_user.into();
        
        // Update fields
        active_model.email = Set(user.email);
        active_model.password_hash = Set(user.password_hash);
        active_model.username = Set(user.username);
        active_model.first_name = Set(user.first_name);
        active_model.last_name = Set(user.last_name);
        active_model.avatar_url = Set(user.avatar_url);
        active_model.is_verified = Set(user.is_verified);
        active_model.is_active = Set(user.is_active);
        active_model.metadata = Set(user.metadata);
        active_model.updated_at = Set(user.updated_at);
        active_model.deleted_at = Set(user.deleted_at);
        active_model.role_id = Set(user.role_id); // Update role_id
        
        let result = active_model
            .update(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(result)
    }
    
    async fn delete_user(&self, id: Uuid) -> Result<(), PostgresRepositoryError> {
        let user = self.find_user_by_id(id).await?
            .ok_or_else(|| PostgresRepositoryError::NotFound(format!("User with id {id} not found")))?;
        
        user.delete(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn list_users(&self, offset: u64, limit: u64) -> Result<Vec<UserModel>, PostgresRepositoryError> {
        let users = UsersEntity::find()
            .offset(offset)
            .limit(limit)
            .all(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(users)
    }
    
    async fn count_users(&self) -> Result<u64, PostgresRepositoryError> {
        let count = UsersEntity::find()
            .count(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(count)
    }
    
    async fn find_role_by_id(&self, id: Uuid) -> Result<Option<RoleModel>, PostgresRepositoryError> {
        let role = RolesEntity::find_by_id(id)
            .one(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(role)
    }
    
    async fn find_role_by_name(&self, name: &str) -> Result<Option<RoleModel>, PostgresRepositoryError> {
        let role = RolesEntity::find()
            .filter(imphnen_entities::seaorm::auth::roles::Column::Name.eq(name))
            .one(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(role)
    }
    
    async fn create_role(&self, role: RoleModel) -> Result<RoleModel, PostgresRepositoryError> {
        let active_model = imphnen_entities::seaorm::auth::roles::ActiveModel {
            id: Set(role.id),
            name: Set(role.name),
            description: Set(role.description),
            permissions: Set(role.permissions),
            is_system_role: Set(role.is_system_role),
            is_default: Set(role.is_default),
            created_at: Set(role.created_at),
            updated_at: Set(role.updated_at),
            deleted_at: Set(role.deleted_at),
        };
        
        let result = active_model
            .insert(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(result)
    }
    
    async fn update_role(&self, id: Uuid, role: RoleModel) -> Result<RoleModel, PostgresRepositoryError> {
        let existing_role = self.find_role_by_id(id).await?
            .ok_or_else(|| PostgresRepositoryError::NotFound(format!("Role with id {id} not found")))?;
        
        let mut active_model: imphnen_entities::seaorm::auth::roles::ActiveModel = existing_role.into();
        
        active_model.name = Set(role.name);
        active_model.description = Set(role.description);
        active_model.permissions = Set(role.permissions);
        active_model.is_system_role = Set(role.is_system_role);
        active_model.is_default = Set(role.is_default);
        active_model.updated_at = Set(role.updated_at);
        active_model.deleted_at = Set(role.deleted_at);
        
        let result = active_model
            .update(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(result)
    }
    
    async fn delete_role(&self, id: Uuid) -> Result<(), PostgresRepositoryError> {
        let role = self.find_role_by_id(id).await?
            .ok_or_else(|| PostgresRepositoryError::NotFound(format!("Role with id {id} not found")))?;
        
        role.delete(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn list_roles(&self) -> Result<Vec<RoleModel>, PostgresRepositoryError> {
        let roles = RolesEntity::find()
            .all(&self.connection.conn)
            .await
            .map_err(PostgresRepositoryError::DatabaseError)?;
        
        Ok(roles)
    }
}

/// Conversion utilities for data transformation between different formats
pub mod conversion_utils {
    use super::*;
    use serde_json::Value;
    
    /// Convert JSON value to PostgreSQL-compatible format
    pub fn json_to_pg_json(json: Value) -> Result<serde_json::Value, PostgresRepositoryError> {
        Ok(json)
    }
    
    /// Convert PostgreSQL JSON to standard JSON value
    pub fn pg_json_to_json(pg_json: serde_json::Value) -> Result<Value, PostgresRepositoryError> {
        Ok(pg_json)
    }
    
    /// Convert string to PostgreSQL UUID format
    pub fn string_to_uuid(uuid_str: &str) -> Result<Uuid, PostgresRepositoryError> {
        Uuid::parse_str(uuid_str)
            .map_err(|e| PostgresRepositoryError::ConversionError(format!("Invalid UUID: {e}")))
    }
    
    /// Convert DateTime to PostgreSQL timestamp format
    pub fn datetime_to_pg_timestamp(dt: DateTime<Utc>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }
    
    /// Convert PostgreSQL timestamp string to DateTime
    pub fn pg_timestamp_to_datetime(pg_timestamp: &str) -> Result<DateTime<Utc>, PostgresRepositoryError> {
        DateTime::parse_from_rfc3339(pg_timestamp)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| PostgresRepositoryError::ConversionError(format!("Invalid timestamp: {e}")))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_conversion_utils() {
        // Test UUID conversion
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let uuid = conversion_utils::string_to_uuid(uuid_str).unwrap();
        assert_eq!(uuid.to_string(), uuid_str);
        
        // Test timestamp conversion
        let now = Utc::now();
        let pg_timestamp = conversion_utils::datetime_to_pg_timestamp(now);
        assert!(!pg_timestamp.is_empty());
    }
    
    #[test]
    fn test_postgres_repository_error() {
        let error = PostgresRepositoryError::NotFound("Test error".to_string());
        assert_eq!(error.to_string(), "Entity not found: Test error");
        
        let error = PostgresRepositoryError::ValidationError("Invalid input".to_string());
        assert_eq!(error.to_string(), "Validation error: Invalid input");
    }
}