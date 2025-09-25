pub mod admin_teams_controller;
pub mod teams_controller;
pub mod teams_dto;
pub mod teams_repository;
pub mod teams_schema;
pub mod teams_service;

pub use admin_teams_controller::{admin_teams_router, get_all_teams as admin_get_all_teams, get_team_by_id as admin_get_team_by_id, get_team_members as admin_get_team_members, create_team as admin_create_team, update_team as admin_update_team, delete_team as admin_delete_team, invite_team_members as admin_invite_team_members};
pub use teams_controller::{teams_router as user_teams_router, get_team_list, get_team_by_id as user_get_team_by_id, get_team_members as user_get_team_members};
pub use teams_dto::*;
pub use teams_repository::*;
pub use teams_schema::*;
pub use teams_service::*;

use axum::Router;

pub fn teams_router() -> Router {
	Router::new()
		// Public routes
		.merge(teams_controller::teams_router())
		// Admin routes - prefixed with /admin to avoid route conflicts
		.nest("/admin", admin_teams_controller::admin_teams_router())
}