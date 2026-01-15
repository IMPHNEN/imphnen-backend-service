//! Examples and usage patterns for PostgreSQL integration with SeaORM

use std::sync::Arc;
use uuid::Uuid;
use sea_orm::{EntityTrait, ColumnTrait, QueryFilter, DatabaseConnection};

use crate::{
    postgres::{PostgresConnection, PostgresConfig, PostgresError},
    AppState, AppStatePostgresExt,
    imphnen_entities::seaorm::auth::users::Entity as UserEntity,
    imphnen_entities::seaorm::auth::users::Model as UserModel,
    imphnen_entities::seaorm::auth::users::ActiveModel as UserActiveModel,
    imphnen_entities::seaorm::common::enums::ResourceEnum,
};

/// Example: Basic PostgreSQL connection usage
pub async fn basic_postgres_usage_example() -> Result<(), PostgresError> {
    // Load configuration from environment variables
    let config = PostgresConfig::from_env()?;
    
    // Create PostgreSQL connection
    let postgres_conn = PostgresConnection::new(config).await?;
    
    // Example: Execute a raw SQL query
    let statement = sea_orm::Statement::from_string(
        sea_orm::DatabaseBackend::Postgres,
        "SELECT version()".into(),
    );
    
    let result = postgres_conn.execute(statement).await?;
    println!("PostgreSQL version query result: {:?}", result);
    
    Ok(())
}

/// Example: PostgreSQL integration with AppState
pub async fn app_state_integration_example(
    postgres_config: PostgresConfig,
) -> Result<AppState, PostgresError> {
    // Create AppState with PostgreSQL connection
    let app_state = AppState::new(
        postgres_config,
        Arc::new(dummy_user_lookup_service()),
        Arc::new(dummy_auth_repository()),
    ).await?;

    // Access PostgreSQL connection from AppState
    let postgres_conn = app_state.postgres_connection();
    println!("Successfully accessed PostgreSQL connection from AppState");

    Ok(app_state)
}

/// Example: Repository pattern with PostgreSQL (simplified)
pub struct UserRepository {
    postgres_conn: Arc<PostgresConnection>,
}

impl UserRepository {
    /// Create a new UserRepository
    pub fn new(postgres_conn: Arc<PostgresConnection>) -> Self {
        Self { postgres_conn }
    }

    /// Get user by email
    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<UserModel>, PostgresError> {
        let users = UserEntity::find()
            .filter(UserEntity::email.eq(email))
            .all(&self.postgres_conn.conn)
            .await
            .map_err(|e| PostgresError::ConnectionError(e.into()))?;
        
        Ok(users.into_iter().next())
    }

    /// Create a new user
    pub async fn create_user(&self, user: UserActiveModel) -> Result<UserModel, PostgresError> {
        let result = user.save(&self.postgres_conn.conn)
            .await
            .map_err(|e| PostgresError::ConnectionError(e.into()))?;
        
        Ok(result)
    }
}


/// Example: Service layer using PostgreSQL repository
pub struct UserService {
    user_repository: UserRepository,
}

impl UserService {
    /// Create a new UserService
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository }
    }
    
    /// Get user by email with additional business logic
    pub async fn get_user_by_email_with_logging(&self, email: &str) -> Result<Option<UserModel>, PostgresError> {
        println!("Attempting to find user with email: {}", email);
        
        let user = self.user_repository.get_user_by_email(email).await?;
        
        if let Some(user) = &user {
            println!("Found user: {}", user.username);
        } else {
            println!("User not found with email: {}", email);
        }
        
        Ok(user)
    }
}

/// Dummy implementations for dependencies
fn dummy_user_lookup_service() -> impl crate::services::UserLookupService {
    struct DummyUserLookupService;
    impl crate::services::UserLookupService for DummyUserLookupService {
        async fn lookup_user(&self, _: &str) -> Result<Option<crate::imphnen_entities::User>, String> {
            Ok(None)
        }
    }
    DummyUserLookupService
}

fn dummy_auth_repository() -> impl crate::services::AuthRepositoryTrait {
    struct DummyAuthRepository;
    impl crate::services::AuthRepositoryTrait for DummyAuthRepository {
        async fn verify_credentials(&self, _: &str, _: &str) -> Result<bool, String> {
            Ok(false)
        }
    }
    DummyAuthRepository
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::MockDatabaseConnection;
    
    #[tokio::test]
    async fn test_postgres_config_from_env() {
        // This test doesn't actually check environment variables
        // It just ensures the method doesn't panic
        let result = PostgresConfig::from_env();
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_user_repository_create() {
        let mock_conn = MockDatabaseConnection::new();
        let postgres_conn = Arc::new(PostgresConnection {
            conn: mock_conn,
            config: PostgresConfig::default(),
        });
        
        let user_repo = UserRepository::new(postgres_conn);
        
        // We can't actually test the create_user method without a real database
        // but we can test that it compiles and doesn't panic
        let user_active_model = UserActiveModel {
            id: sea_orm::Set(Uuid::new_v4()),
            email: sea_orm::Set("test@example.com".into()),
            username: sea_orm::Set("testuser".into()),
            // Add other required fields as needed
            ..Default::default()
        };
        
        let result = user_repo.create_user(user_active_model).await;
        assert!(result.is_err()); // Expected to fail with mock connection
    }
}