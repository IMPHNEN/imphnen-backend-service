// API entry point using PostgreSQL (SurrealDB migration complete)
// This file has been updated to use SeaORM with PostgreSQL instead of SurrealDB
use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	axum_init(|postgres_db| async {
		// Gateway service now uses PostgreSQL exclusively (SeaORM)
		// SurrealDB dependencies have been completely removed
		gateway_service(postgres_db).await
	})
	.await;
}
