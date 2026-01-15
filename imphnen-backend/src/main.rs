use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	tracing_subscriber::fmt::init();

	let _ = axum_init(|postgres_conn| async {
		// PostgreSQL is now the primary database - SurrealDB has been completely removed
		gateway_service(postgres_conn).await
	})
	.await;
}
