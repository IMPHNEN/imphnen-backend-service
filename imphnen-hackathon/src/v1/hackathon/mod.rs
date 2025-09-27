use axum::Router;

pub mod hackathon_controller;
pub mod hackathon_dto;
pub mod hackathon_repository;
pub mod hackathon_schema;
pub mod hackathon_service;

// Export types and functions
pub use hackathon_dto::*;
pub use hackathon_repository::HackathonRepository;
pub use hackathon_schema::*;
pub use hackathon_service::{HackathonService, HackathonServiceTrait};

// Export controller functions
pub use hackathon_controller::*;

pub fn hackathon_router() -> Router {
    hackathon_controller::hackathon_routes()
}