use crate::postgres::{
	AppStatePostgresExt, PostgresConfig, PostgresConnection, PostgresError,
};
use crate::services::{AuthRepositoryTrait, UserLookupService};
use std::sync::Arc;

pub struct PostgresClients {
	pub main: Arc<PostgresConnection>,
	pub read_only: Option<Arc<PostgresConnection>>,
	pub test: Option<Arc<PostgresConnection>>,
}

impl PostgresClients {
	pub fn new(main: Arc<PostgresConnection>) -> Self {
		Self {
			main,
			read_only: None,
			test: None,
		}
	}

	pub fn with_read_only(mut self, read_only: Arc<PostgresConnection>) -> Self {
		self.read_only = Some(read_only);
		self
	}

	pub fn with_test(mut self, test: Arc<PostgresConnection>) -> Self {
		self.test = Some(test);
		self
	}
}

#[derive(Clone)]
pub struct AppState {
	pub postgres_connection: Arc<PostgresConnection>,
	pub user_lookup_service: Arc<dyn UserLookupService>,
	pub auth_repository: Arc<dyn AuthRepositoryTrait>,
}

impl AppState {
	pub async fn new(
		postgres_config: PostgresConfig,
		user_lookup_service: Arc<dyn UserLookupService>,
		auth_repository: Arc<dyn AuthRepositoryTrait>,
	) -> Result<Self, PostgresError> {
		let postgres_connection = PostgresConnection::new(postgres_config).await?;
		Ok(Self {
			postgres_connection: Arc::new(postgres_connection),
			user_lookup_service,
			auth_repository,
		})
	}
}

impl AppStatePostgresExt for AppState {
	fn postgres_connection(&self) -> &PostgresConnection {
		&self.postgres_connection
	}
}
