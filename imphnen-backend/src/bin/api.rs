use axum::Router;
use imphnen_gateway::gateway_service;
use imphnen_libs::axum_init;

#[tokio::main]
async fn main() {
	env_logger::init();
	axum_init(|surrealdb_ws, surrealdb_mem| async {
		let app = gateway_service(surrealdb_ws, surrealdb_mem).await;
        let mut router = Router::new();
        router = router.nest("/api/v1/auth", imphnen_iam::v1::auth::auth_router());
        app.merge(router)
	})
	.await;
}
