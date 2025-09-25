pub mod admin_teams_controller;
pub mod teams_controller;
pub mod teams_dto;
pub mod teams_repository;
pub mod teams_schema;
pub mod teams_service;

pub use admin_teams_controller::*;
pub use teams_controller::*;
pub use teams_dto::*;
pub use teams_repository::*;
pub use teams_schema::*;
pub use teams_service::*;

use axum::Router;
use crate::teams_controller::*;
use crate::admin_teams_controller::*;

pub fn teams_router() -> Router {
	Router::new()
		// Public routes
		.merge(teams_controller::teams_router())
		// Admin routes
		.merge(admin_teams_controller::admin_teams_router())
}