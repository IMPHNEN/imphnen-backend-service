use axum::Router;

pub mod hackathon;
pub mod notifications;
pub mod registrations;

// Export the router function from hackathon module
pub use hackathon::hackathon_router;
pub use notifications::notifications_router;
pub use registrations::registrations_router;

// Main route constructor
pub fn hackathon_protected_routes() -> Router {
    // Protected routes include the main hackathon router (create/update/delete) and
    // a protected route for updating submission status.
    use hackathon::hackathon_controller::{update_submission_status, get_admin_hackathon_results};
    Router::new()
        .nest("/hackathons", hackathon_router())
        .route("/hackathons/submissions/update/{id}/status", axum::routing::patch(update_submission_status))
        .route("/hackathons/{hackathon_id}/admin/results", axum::routing::get(get_admin_hackathon_results))
        .merge(registrations_router())
        .merge(notifications_router())
}

// Public routes for hackathons (only listing and retrieving)
pub fn hackathon_public_routes() -> Router {
    use hackathon::hackathon_controller::{
        list_hackathons,
        search_hackathons,
        get_user_hackathon_submissions,
        get_public_hackathon_results,
    };

    Router::new()
        .nest("/hackathons", Router::new()
            .route("/", axum::routing::get(list_hackathons))
            .route("/{id}/results", axum::routing::get(get_public_hackathon_results))
            .route("/search", axum::routing::post(search_hackathons))
        )
        .route("/users/{user_id}/hackathon-submissions", axum::routing::get(get_user_hackathon_submissions))
}