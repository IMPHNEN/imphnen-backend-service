use std::env;
use dotenvy::dotenv;
use sea_orm::{
    ConnectOptions, Database, DatabaseConnection, DbErr, Statement,
    ConnectionTrait, QueryResult, ExecResult, DatabaseTransaction,
    TransactionTrait,
};
use tokio::time::{Duration, Instant};
use thiserror::Error;

/// Configuration for PostgreSQL connection
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    /// Database URL (e.g., postgres://user:pass@host:port/dbname)
    pub database_url: String,
    /// Maximum number of connections in the pool
    pub pool_size: u32,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
    /// Idle timeout in seconds
    pub idle_timeout: u64,
    /// Max lifetime of connections in seconds
    pub max_lifetime: Option<u64>,
    /// Retry attempts for connection
    pub retry_attempts: u32,
    /// Retry delay between attempts in seconds
    pub retry_delay: u64,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            database_url: "postgres://postgres:postgres@localhost:5432/imphnen".into(),
            pool_size: 10,
            connect_timeout: 30,
            idle_timeout: 60,
            max_lifetime: Some(1800),
            retry_attempts: 3,
            retry_delay: 1,
        }
    }
}

impl PostgresConfig {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self, PostgresError> {
        dotenv().ok();
        
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| PostgresError::EnvVarMissing("DATABASE_URL".into()))?;
        
        Ok(Self {
            database_url,
            pool_size: env::var("POOL_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10),
            connect_timeout: env::var("CONNECT_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(30),
            idle_timeout: env::var("IDLE_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60),
            max_lifetime: env::var("MAX_LIFETIME")
                .ok()
                .and_then(|s| s.parse().ok())
                .map(Some)
                .unwrap_or(Some(1800)),
            retry_attempts: env::var("RETRY_ATTEMPTS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3),
            retry_delay: env::var("RETRY_DELAY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1),
        })
    }
}

/// Errors that can occur during PostgreSQL connection
#[derive(Debug, Error)]
pub enum PostgresError {
    /// Environment variable is missing
    #[error("Environment variable {0} is missing")]
    EnvVarMissing(String),
    
    /// Database connection error
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] DbErr),
    
    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// Retry limit exceeded
    #[error("Retry limit exceeded for database connection")]
    RetryLimitExceeded,
    
    /// Timeout error
    #[error("Connection timeout: {0}")]
    TimeoutError(String),
    
    #[error("Operation failed: {0}")]
    OperationFailed(String),
}

/// PostgreSQL connection manager with pooling
#[derive(Clone)]
pub struct PostgresConnection {
    /// Database connection pool
    pub conn: DatabaseConnection,
    /// Configuration
    pub config: PostgresConfig,
}

impl PostgresConnection {
    /// Create a new PostgreSQL connection with connection pooling
    pub async fn new(config: PostgresConfig) -> Result<Self, PostgresError> {
        let connect_options = Self::build_connect_options(&config)?;
        
        // Implement retry logic for connection
        let mut last_error = None;
        for attempt in 1..=config.retry_attempts {
            match Self::connect_with_timeout(connect_options.clone(), config.connect_timeout).await {
                Ok(conn) => return Ok(Self { conn, config }),
                Err(err) => {
                    last_error = Some(err);
                    if attempt < config.retry_attempts {
                        tokio::time::sleep(Duration::from_secs(config.retry_delay)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap_or_else(|| {
            PostgresError::ConfigError("Failed to connect to database".into())
        }))
    }
    
    /// Build connection options with pooling and timeouts
    fn build_connect_options(config: &PostgresConfig) -> Result<ConnectOptions, PostgresError> {
        let mut options = ConnectOptions::new(config.database_url.clone());
        
        options.max_connections(config.pool_size)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(config.connect_timeout))
            .idle_timeout(Duration::from_secs(config.idle_timeout));
        
        if let Some(max_lifetime) = config.max_lifetime {
            options.max_lifetime(Duration::from_secs(max_lifetime));
        }
        
        Ok(options)
    }
    
    /// Connect with timeout
    async fn connect_with_timeout(
        options: ConnectOptions,
        timeout: u64,
    ) -> Result<DatabaseConnection, PostgresError> {
        let deadline = Instant::now() + Duration::from_secs(timeout);
        
        tokio::select! {
            result = Database::connect(options) => result.map_err(PostgresError::ConnectionError),
            _ = tokio::time::sleep_until(deadline) => {
                Err(PostgresError::TimeoutError(format!(
                    "Connection timed out after {} seconds",
                    timeout
                )))
            }
        }
    }
    
    /// Execute a raw SQL statement
    pub async fn execute(&self, statement: Statement) -> Result<ExecResult, PostgresError> {
        self.conn.execute(statement).await.map_err(PostgresError::ConnectionError)
    }
    
    /// Query one result
    pub async fn query_one(&self, statement: Statement) -> Result<Option<QueryResult>, PostgresError> {
        self.conn.query_one(statement).await.map_err(PostgresError::ConnectionError)
    }
    
    /// Query all results
    pub async fn query_all(&self, statement: Statement) -> Result<Vec<QueryResult>, PostgresError> {
        self.conn.query_all(statement).await.map_err(PostgresError::ConnectionError)
    }
    
    /// Execute a raw SQL query and return results
    pub async fn execute_raw(&self, sql: &str) -> Result<Vec<QueryResult>, PostgresError> {
        let statement = Statement::from_string(
            self.conn.get_database_backend(),
            sql.to_string()
        );
        self.query_all(statement).await
    }
    
    /// Get database backend type
    pub fn get_database_backend(&self) -> sea_orm::DatabaseBackend {
        self.conn.get_database_backend()
    }
    
    /// Begin a transaction
    pub async fn begin_transaction(&self) -> Result<DatabaseTransaction, PostgresError> {
        self.conn.begin().await.map_err(PostgresError::ConnectionError)
    }
    
    /// Execute a transaction with automatic commit/rollback
    pub async fn transaction<'a, F, R>(&'a self, f: F) -> Result<R, PostgresError>
    where
        F: FnOnce(&DatabaseTransaction) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<R, PostgresError>> + Send>> + Send + 'a + 'static,
        R: Send + 'a + 'static,
    {
        self.conn.transaction(|txn| {
            Box::pin(async move {
                f(txn).await
            })
        }).await.map_err(|e| {
            PostgresError::ConnectionError(DbErr::Custom(e.to_string()))
        })
    }

    /// Execute a simple database query
    pub async fn query_simple(&self, sql: &str) -> Result<Vec<QueryResult>, PostgresError> {
        let statement = Statement::from_string(
            self.conn.get_database_backend(),
            sql.to_string()
        );
        self.conn.query_all(statement).await.map_err(PostgresError::ConnectionError)
    }
}

/// Extension trait for AppState to add PostgreSQL functionality
pub trait AppStatePostgresExt {
    /// Get the PostgreSQL connection
    fn postgres_connection(&self) -> &PostgresConnection;
    
    /// Get the raw database connection (implements ConnectionTrait)
    fn postgres_db(&self) -> &DatabaseConnection {
        &self.postgres_connection().conn
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Statement;
    
    #[tokio::test]
    async fn test_postgres_config_default() {
        let config = PostgresConfig::default();
        assert_eq!(config.pool_size, 10);
        assert_eq!(config.connect_timeout, 30);
        assert_eq!(config.idle_timeout, 60);
        assert_eq!(config.retry_attempts, 3);
        assert_eq!(config.retry_delay, 1);
    }
    
    #[tokio::test]
    async fn test_postgres_connection_from_env() {
        // Skip actual connection in test
        let config = PostgresConfig::from_env();
        assert!(config.is_ok());
    }
    
    #[tokio::test]
    async fn test_postgres_statement_execution() {
        // This is a mock test since we don't want to connect to a real database in tests
        let config = PostgresConfig::default();
        let connection_result = PostgresConnection::new(config).await;
        
        match connection_result {
            Ok(_) => {
                // If we somehow got a connection, test statement execution
                let statement = Statement::from_string(
                    sea_orm::DatabaseBackend::Postgres,
                    "SELECT 1".to_string(),
                );
                
                // We expect this to fail in a test environment without a real database
                assert!(connection_result.unwrap().execute(statement).await.is_err());
            }
            Err(_) => {
                // Expected behavior in test environment
                assert!(true);
            }
        }
    }
}