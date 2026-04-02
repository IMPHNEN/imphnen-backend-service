use super::connection::{PostgresConnection, PostgresError};
use sea_orm::{
	ConnectionTrait, DatabaseTransaction, DbErr, ExecResult, QueryResult, Statement,
	TransactionTrait,
};

impl PostgresConnection {
	pub async fn execute(
		&self,
		statement: Statement,
	) -> Result<ExecResult, PostgresError> {
		self
			.conn
			.execute(statement)
			.await
			.map_err(PostgresError::ConnectionError)
	}

	pub async fn query_one(
		&self,
		statement: Statement,
	) -> Result<Option<QueryResult>, PostgresError> {
		self
			.conn
			.query_one(statement)
			.await
			.map_err(PostgresError::ConnectionError)
	}

	pub async fn query_all(
		&self,
		statement: Statement,
	) -> Result<Vec<QueryResult>, PostgresError> {
		self
			.conn
			.query_all(statement)
			.await
			.map_err(PostgresError::ConnectionError)
	}

	pub async fn execute_raw(
		&self,
		sql: &str,
	) -> Result<Vec<QueryResult>, PostgresError> {
		let statement =
			Statement::from_string(self.conn.get_database_backend(), sql.to_string());
		self.query_all(statement).await
	}

	pub async fn begin_transaction(
		&self,
	) -> Result<DatabaseTransaction, PostgresError> {
		self
			.conn
			.begin()
			.await
			.map_err(PostgresError::ConnectionError)
	}

	pub async fn transaction<'a, F, R>(&'a self, f: F) -> Result<R, PostgresError>
	where
		F: FnOnce(
				&DatabaseTransaction,
			) -> std::pin::Pin<
				Box<dyn std::future::Future<Output = Result<R, PostgresError>> + Send>,
			> + Send
			+ 'a + 'static,
		R: Send + 'a + 'static,
	{
		self
			.conn
			.transaction(|txn| Box::pin(async move { f(txn).await }))
			.await
			.map_err(|e| PostgresError::ConnectionError(DbErr::Custom(e.to_string())))
	}

	pub async fn query_simple(
		&self,
		sql: &str,
	) -> Result<Vec<QueryResult>, PostgresError> {
		let statement =
			Statement::from_string(self.conn.get_database_backend(), sql.to_string());
		self
			.conn
			.query_all(statement)
			.await
			.map_err(PostgresError::ConnectionError)
	}
}

pub trait AppStatePostgresExt {
	fn postgres_connection(&self) -> &PostgresConnection;

	fn postgres_db(&self) -> &sea_orm::DatabaseConnection {
		&self.postgres_connection().conn
	}
}
