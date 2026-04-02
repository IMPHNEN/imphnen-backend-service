use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	axum_init(|postgres_db| async {
		gateway_service(postgres_db).await
	})
	.await;
}
