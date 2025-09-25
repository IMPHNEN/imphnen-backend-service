pub mod teams_controller;
pub mod teams_dto;
pub mod teams_repository;
pub mod teams_schema;
pub mod teams_service;

pub use teams_controller::*;
pub use teams_dto::*;
pub use teams_repository::*;
pub use teams_schema::*;
pub use teams_service::*;

use axum::Router;

pub fn teams_router() -> Router {
	Router::new()
}