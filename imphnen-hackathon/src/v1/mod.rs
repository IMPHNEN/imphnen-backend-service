use axum::Router;

pub mod hackathon;

// Export the router function from hackathon module
pub use hackathon::hackathon_router;

// Main route constructor
pub fn hackathon_protected_routes() -> Router {
    Router::new().nest("/hackathons", hackathon_router())
}

// Public routes for hackathons (only listing and retrieving)
pub fn hackathon_public_routes() -> Router {
    use hackathon::hackathon_controller::{list_hackathons, get_hackathon};
    Router::new()
        .nest("/hackathons", Router::new()
            .route("/", axum::routing::get(list_hackathons))
            .route("/{id}", axum::routing::get(get_hackathon))
        )
}