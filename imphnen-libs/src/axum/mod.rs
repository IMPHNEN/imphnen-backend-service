pub mod app_state;
pub mod validated_json;
pub mod zod_validate;

pub use app_state::{AppState, PostgresClients};
pub use validated_json::ValidatedJson;
pub use zod_validate::ZodValidate;

use crate::environment::ENV;
use crate::postgres::{PostgresConfig, PostgresConnection, PostgresError};
use axum::{Router, serve};
use std::sync::Arc;
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub struct ServerConfig {
	pub port: u16,
	pub host: String,
	pub max_request_size: usize,
	pub request_timeout: u64,
	pub worker_threads: usize,
	pub enable_logging: bool,
	pub enable_tracing: bool,
}

impl Default for ServerConfig {
	fn default() -> Self {
		Self {
			port: 3000,
			host: "0.0.0.0".to_string(),
			max_request_size: 10 * 1024 * 1024,
			request_timeout: 30,
			worker_threads: std::thread::available_parallelism()
				.map(|n| n.get())
				.unwrap_or(4),
			enable_logging: true,
			enable_tracing: true,
		}
	}
}

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

	log::info!("Starting server initialization with PostgreSQL support");

	let main_connection = match PostgresConnection::new(postgres_config.clone()).await
	{
		Ok(conn) => {
			log::info!("Main PostgreSQL connection established successfully");
			Arc::new(conn)
		}
		Err(e) => {
			log::error!("Failed to establish main PostgreSQL connection: {}", e);
			return Err(ServerInitError::DatabaseConnectionFailed(e));
		}
	};

	match test_postgres_connection(&main_connection).await {
		Ok(()) => log::info!("PostgreSQL connection test passed"),
		Err(e) => {
			log::error!("PostgreSQL connection test failed: {}", e);
			return Err(ServerInitError::DatabaseConnectionFailed(e));
		}
	}

	let postgres_clients = PostgresClients::new(main_connection);

	log::info!("PostgreSQL clients initialized successfully");

	let router = router_fn(postgres_clients).await;

	let port = server_config.port;
	let host = server_config.host.clone();
	let addr = format!("{host}:{port}");
	let socket_addr: SocketAddr = addr.parse().map_err(|e| {
		ServerInitError::ConfigurationError(format!("Invalid address '{addr}': {e}"))
	})?;

	log::info!("Configuring server to listen on {}", socket_addr);

	let listener = TcpListener::bind(&socket_addr).await.map_err(|e| {
		ServerInitError::NetworkBindingFailed(format!(
			"Failed to bind to {socket_addr}: {e}"
		))
	})?;

	log::info!("Server starting on {}", socket_addr);

	let shutdown_handle = setup_graceful_shutdown();

	let server_handle = tokio::spawn(async move {
		if let Err(err) = serve(listener, router).await {
			log::error!("Server encountered an error: {}", err);
			Err(ServerInitError::ServerStartupFailed(err.to_string()))
		} else {
			Ok(())
		}
	});

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

pub async fn axum_init<F, Fut>(router_fn: F) -> Result<(), ServerInitError>
where
	F: FnOnce(PostgresClients) -> Fut,
	Fut: Future<Output = Router>,
{
	let postgres_config = PostgresConfig::from_env().map_err(|e| {
		ServerInitError::ConfigurationError(format!(
			"Failed to load PostgreSQL config: {e}"
		))
	})?;

	let server_config = ServerConfig {
		port: ENV.port,
		..ServerConfig::default()
	};

	axum_init_advanced(router_fn, Some(server_config), postgres_config).await
}

async fn test_postgres_connection(
	connection: &Arc<PostgresConnection>,
) -> Result<(), PostgresError> {
	use sea_orm::DbErr;
	let test_query = sea_orm::Statement::from_string(
		connection.get_database_backend(),
		"SELECT 1 as test_value".to_string(),
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
					"Connection test query returned unexpected result".to_string(),
				)))
			}
		}
		None => Err(PostgresError::ConnectionError(sea_orm::DbErr::Custom(
			"Connection test query returned no results".to_string(),
		))),
	}
}

async fn setup_graceful_shutdown() {
	use tokio::signal;

	match signal::ctrl_c().await {
		Ok(()) => {
			log::info!("Received Ctrl+C, initiating graceful shutdown");
		}
		Err(err) => {
			log::error!("Unable to listen for shutdown signal: {}", err);
			std::future::pending::<()>().await;
		}
	}
}
