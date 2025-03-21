use imphnen_backend_service::{apps, axum_init};

#[tokio::main]
async fn main() {
	axum_init(|db, redis| async { apps(db, redis).await }).await;
}
