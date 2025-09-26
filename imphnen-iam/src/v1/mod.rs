use axum::Router;

pub mod auth;
pub mod permissions;
pub mod roles;
pub mod teams;
pub mod users;

// Export only the essential router functions from each module
pub use auth::auth_router;
pub use permissions::{permissions_router, permissions_dto, permissions_service, permissions_guard};
pub use roles::{roles_router, roles_service};
pub use teams::teams_router;
pub use users::users_router;

// Main route constructors
pub fn iam_public_routes() -> Router {
	Router::new().nest("/auth", auth_router())
}

pub fn iam_protected_routes() -> Router {
	Router::new()
		.nest("/users", users_router())
		.nest("/roles", roles_router())
		.nest("/permissions", permissions_router())
		.nest("/teams", teams_router())
}
