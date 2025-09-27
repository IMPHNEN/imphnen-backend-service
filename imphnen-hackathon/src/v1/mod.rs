use axum::Router;

pub mod hackathon;

// Export the router function from hackathon module
pub use hackathon::hackathon_router;

// Main route constructor
pub fn hackathon_protected_routes() -> Router {
    Router::new().nest("/hackathons", hackathon_router())
}