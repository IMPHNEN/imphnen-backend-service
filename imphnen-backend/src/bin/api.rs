use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		gateway_service(surrealdb_ws, surrealdb_mem).await
	})
	.await;
}
