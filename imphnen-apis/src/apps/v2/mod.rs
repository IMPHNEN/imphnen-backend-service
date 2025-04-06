use axum::Router;

pub mod auth;
pub use auth::*;

pub async fn routes() -> Router {
	let public_routes = Router::new().nest("/auth", auth::auth_router());
	Router::new().merge(public_routes)
}
