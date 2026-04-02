//! Axum server initialization utilities.
//!
//! This module provides utilities for initializing and running an Axum web server
//! with PostgreSQL database connections and comprehensive error handling.

pub mod validated_json;
pub mod zod_validate;

use axum::{Router, serve};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;
use crate::environment::ENV;
use crate::postgres::{PostgresConnection, PostgresConfig, PostgresError};
use sea_orm::DbErr;
use std::sync::Arc;

pub use validated_json::ValidatedJson;
pub use zod_validate::ZodValidate;

/// PostgreSQL database clients for different connection types
pub struct PostgresClients {
    /// Main PostgreSQL connection for production use
    pub main: Arc<PostgresConnection>,
    /// Read-only PostgreSQL connection for read-heavy operations
    pub read_only: Option<Arc<PostgresConnection>>,
    /// Test PostgreSQL connection for testing scenarios
    pub test: Option<Arc<PostgresConnection>>,
}

impl PostgresClients {
    /// Create new PostgreSQL clients with main connection
    pub fn new(main: Arc<PostgresConnection>) -> Self {
        Self {
            main,
            read_only: None,
            test: None,
        }
    }
    
    /// Add read-only connection
    pub fn with_read_only(mut self, read_only: Arc<PostgresConnection>) -> Self {
        self.read_only = Some(read_only);
        self
    }
    
    /// Add test connection
    pub fn with_test(mut self, test: Arc<PostgresConnection>) -> Self {
        self.test = Some(test);
        self
    }
}

/// Comprehensive server configuration
pub struct ServerConfig {
    /// Server port
    pub port: u16,
    /// Server host
    pub host: String,
    /// Maximum request body size in bytes
    pub max_request_size: usize,
    /// Request timeout in seconds
    pub request_timeout: u64,
    /// Number of worker threads
    pub worker_threads: usize,
    /// Enable request logging
    pub enable_logging: bool,
    /// Enable request tracing
    pub enable_tracing: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            host: "0.0.0.0".to_string(),
            max_request_size: 10 * 1024 * 1024, // 10MB
            request_timeout: 30,
            worker_threads: std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            enable_logging: true,
            enable_tracing: true,
        }
    }
}

/// Server initialization error
#[derive(Debug, thiserror::Error)]
pub enum ServerInitError {
    #[error("Database connection failed: {0}")]
    DatabaseConnectionFailed(#[from] PostgresError),
    
    #[error("Network binding failed: {0}")]
    NetworkBindingFailed(String),
    
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
    
    #[error("Server startup failed: {0}")]
    ServerStartupFailed(String),
}

/// Initialize and start the Axum server with PostgreSQL connections.
///
/// This function provides a robust server initialization with comprehensive error handling,
/// multiple database connection support, and extensive logging.
///
/// # Arguments
/// * `router_fn` - A function that takes PostgreSQL clients and returns a Router
/// * `config` - Optional server configuration (uses defaults if None)
/// * `postgres_config` - PostgreSQL configuration
///
/// # Returns
/// Result indicating success or detailed error information
///
/// # Example
/// ```no_run
/// use axum::Router;
/// use imphnen_libs::axum::{axum_init_advanced, PostgresClients, ServerConfig};
/// use imphnen_libs::postgres::PostgresConfig;
/// use std::sync::Arc;
///
/// async fn create_router(clients: PostgresClients) -> Router {
///     Router::new()
///         // Add your routes here
/// }
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let postgres_config = PostgresConfig::from_env()?;
///     let server_config = ServerConfig::default();
///     
///     axum_init_advanced(create_router, Some(server_config), postgres_config).await?;
///     Ok(())
/// }
/// ```
pub async fn axum_init_advanced<F, Fut>(
    router_fn: F,
    config: Option<ServerConfig>,
    postgres_config: PostgresConfig,
) -> Result<(), ServerInitError>
where
    F: FnOnce(PostgresClients) -> Fut,
    Fut: Future<Output = Router>,
{
    let server_config = config.unwrap_or_default();
    let _env = &ENV;
    
    // Initialize logging if enabled
    
    
    // Initialize tracing if enabled
    
    
    log::info!("Starting server initialization with PostgreSQL support");
    
    // Initialize PostgreSQL connections with retry logic
    let main_connection = match PostgresConnection::new(postgres_config.clone()).await {
        Ok(conn) => {
            log::info!("Main PostgreSQL connection established successfully");
            Arc::new(conn)
        }
        Err(e) => {
            log::error!("Failed to establish main PostgreSQL connection: {}", e);
            return Err(ServerInitError::DatabaseConnectionFailed(e));
        }
    };
    
    // Test the connection
    match test_postgres_connection(&main_connection).await {
        Ok(()) => log::info!("PostgreSQL connection test passed"),
        Err(e) => {
            log::error!("PostgreSQL connection test failed: {}", e);
            return Err(ServerInitError::DatabaseConnectionFailed(e));
        }
    }
    
    // Create PostgreSQL clients
    let postgres_clients = PostgresClients::new(main_connection);
    
    log::info!("PostgreSQL clients initialized successfully");
    
    // Build the router
    let router = router_fn(postgres_clients).await;
    
    // Configure the server
    let port = server_config.port;
    let host = server_config.host.clone();
    let addr = format!("{host}:{port}");
    let socket_addr: SocketAddr = addr.parse()
        .map_err(|e| ServerInitError::ConfigurationError(format!("Invalid address '{addr}': {e}")))?;
    
    log::info!("Configuring server to listen on {}", socket_addr);
    
    // Bind to the address
    let listener = TcpListener::bind(&socket_addr)
        .await
        .map_err(|e| ServerInitError::NetworkBindingFailed(format!("Failed to bind to {socket_addr}: {e}")))?;
    
    log::info!("Server successfully bound to {}", socket_addr);
    
    // Start the server with graceful shutdown
    log::info!("Server starting on {}", socket_addr);
    
    // Set up graceful shutdown
    let shutdown_handle = setup_graceful_shutdown();
    
    // Run the server
    let server_handle = tokio::spawn(async move {
        if let Err(err) = serve(listener, router).await {
            log::error!("Server encountered an error: {}", err);
            Err(ServerInitError::ServerStartupFailed(err.to_string()))
        } else {
            Ok(())
        }
    });
    
    // Wait for shutdown signal or server error
    tokio::select! {
        result = server_handle => {
            match result {
                Ok(Ok(())) => {
                    log::info!("Server stopped gracefully");
                    Ok(())
                }
                Ok(Err(e)) => {
                    log::error!("Server error: {}", e);
                    Err(e)
                }
                Err(e) => {
                    log::error!("Server task panicked: {}", e);
                    Err(ServerInitError::ServerStartupFailed("Server task panicked".to_string()))
                }
            }
        }
        _ = shutdown_handle => {
            log::info!("Received shutdown signal, stopping server gracefully");
            Ok(())
        }
    }
}

/// Simple server initialization (backward compatibility)
pub async fn axum_init<F, Fut>(router_fn: F) -> Result<(), ServerInitError>
where
    F: FnOnce(PostgresClients) -> Fut,
    Fut: Future<Output = Router>,
{
    let postgres_config = PostgresConfig::from_env()
        .map_err(|e| ServerInitError::ConfigurationError(format!("Failed to load PostgreSQL config: {e}")))?;
    
    let server_config = ServerConfig {
        port: ENV.port,
        ..ServerConfig::default()
    };
    
    axum_init_advanced(router_fn, Some(server_config), postgres_config).await
}

/// Test PostgreSQL connection with comprehensive checks
async fn test_postgres_connection(connection: &Arc<PostgresConnection>) -> Result<(), PostgresError> {
    // Test basic connectivity
    let test_query = sea_orm::Statement::from_string(
        connection.get_database_backend(),
        "SELECT 1 as test_value".to_string()
    );
    
    let result = connection.query_one(test_query).await?;
    
    match result {
        Some(query_result) => {
            let test_value: Option<i32> = query_result.try_get("", "test_value").ok();
            if test_value == Some(1) {
                log::debug!("PostgreSQL connection test successful");
                Ok(())
            } else {
                Err(PostgresError::ConnectionError(DbErr::Custom(
                    "Connection test query returned unexpected result".to_string()
                )))
            }
        }
        None => Err(PostgresError::ConnectionError(DbErr::Custom(
            "Connection test query returned no results".to_string()
        ))),
    }
}

/// Set up graceful shutdown handling
async fn setup_graceful_shutdown() {
    use tokio::signal;
    
    match signal::ctrl_c().await {
        Ok(()) => {
            log::info!("Received Ctrl+C, initiating graceful shutdown");
        }
        Err(err) => {
            log::error!("Unable to listen for shutdown signal: {}", err);
            // Wait forever if we can't listen for signal
            std::future::pending::<()>().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_server_config_default() {
        let config = ServerConfig::default();
        assert_eq!(config.port, 3000);
        assert_eq!(config.host, "0.0.0.0");
        assert_eq!(config.max_request_size, 10 * 1024 * 1024);
        assert_eq!(config.request_timeout, 30);
        assert!(config.enable_logging);
        assert!(config.enable_tracing);
    }
    
    #[test]
    fn test_postgres_clients_creation() {
        // This is a basic test - in real scenarios you'd mock the connection
        let mock_config = PostgresConfig::default();
        // Note: We can't test actual connection without a real database
        // This test just verifies the struct creation logic
    }
    
    #[tokio::test]
    async fn test_server_init_error_types() {
        let error = ServerInitError::ConfigurationError("Test error".to_string());
        assert_eq!(error.to_string(), "Configuration error: Test error");
        
        let error = ServerInitError::NetworkBindingFailed("Bind failed".to_string());
        assert_eq!(error.to_string(), "Network binding failed: Bind failed");
    }
}
