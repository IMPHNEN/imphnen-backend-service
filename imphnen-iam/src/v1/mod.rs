use axum::Router;

pub mod auth;
pub mod permissions;
pub mod roles;
pub mod users;

pub use auth::*;
pub use permissions::*;
pub use roles::*;
pub use users::*;

pub fn iam_public_routes() -> Router {
	Router::new().nest("/auth", auth_router())
}

pub fn iam_protected_routes() -> Router {
	Router::new()
		.nest("/users", users_router())
		.nest("/roles", roles_router())
		.nest("/permissions", permissions_router())
}
