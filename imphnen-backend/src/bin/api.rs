use env_logger;
use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	env_logger::init();
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		gateway_service(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
