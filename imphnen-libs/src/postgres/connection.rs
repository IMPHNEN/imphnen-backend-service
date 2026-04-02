use dotenvy::dotenv;
use sea_orm::{
	ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbErr,
};
use std::env;
use thiserror::Error;
use tokio::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct PostgresConfig {
	pub database_url: String,
	pub pool_size: u32,
	pub connect_timeout: u64,
	pub idle_timeout: u64,
	pub max_lifetime: Option<u64>,
	pub retry_attempts: u32,
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

#[derive(Debug, Error)]
pub enum PostgresError {
	#[error("Environment variable {0} is missing")]
	EnvVarMissing(String),

	#[error("Database connection error: {0}")]
	ConnectionError(#[from] DbErr),

	#[error("Configuration error: {0}")]
	ConfigError(String),

	#[error("Retry limit exceeded for database connection")]
	RetryLimitExceeded,

	#[error("Connection timeout: {0}")]
	TimeoutError(String),

	#[error("Operation failed: {0}")]
	OperationFailed(String),
}

#[derive(Clone)]
pub struct PostgresConnection {
	pub conn: DatabaseConnection,
	pub config: PostgresConfig,
}

impl PostgresConnection {
	pub async fn new(config: PostgresConfig) -> Result<Self, PostgresError> {
		let connect_options = Self::build_connect_options(&config)?;

		let mut last_error = None;
		for attempt in 1..=config.retry_attempts {
			match Self::connect_with_timeout(
				connect_options.clone(),
				config.connect_timeout,
			)
			.await
			{
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

	fn build_connect_options(
		config: &PostgresConfig,
	) -> Result<ConnectOptions, PostgresError> {
		let mut options = ConnectOptions::new(config.database_url.clone());

		options
			.max_connections(config.pool_size)
			.min_connections(5)
			.connect_timeout(Duration::from_secs(config.connect_timeout))
			.idle_timeout(Duration::from_secs(config.idle_timeout));

		if let Some(max_lifetime) = config.max_lifetime {
			options.max_lifetime(Duration::from_secs(max_lifetime));
		}

		Ok(options)
	}

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

	pub fn get_database_backend(&self) -> sea_orm::DatabaseBackend {
		self.conn.get_database_backend()
	}
}
