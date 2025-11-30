use axum::Router;

pub mod auth;
pub mod permissions;
pub mod roles;
pub mod users;

// Export only the essential router functions from each module
pub use auth::auth_router;
pub use permissions::{permissions_router, permissions_dto, permissions_service, permissions_guard};
pub use roles::{roles_router, roles_service};
pub use users::users_router;

// Main route constructors
pub fn iam_public_routes() -> Router {
	Router::new().nest("/auth", auth_router())
}

pub fn iam_protected_routes() -> Router {
	Router::new()
		.nest("/users", users_router())
		.nest("/users/admin", users::admin_users_router())
		.nest("/roles", roles_router())
		.nest("/roles/admin", roles::admin_roles_router())
		.nest("/permissions", permissions_router())
		.nest("/permissions/admin", permissions::admin_permissions_router())
}
