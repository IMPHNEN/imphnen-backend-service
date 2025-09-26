use axum::Router;

pub mod mentors;

/// Creates the main Dimentorin router with all version 1 endpoints
/// Routes:
/// - /mentors -> mentors::mentors_router()
pub fn dimentorin_router() -> Router {
	Router::new()
		.nest("/mentors", mentors::mentors_router())
}

// Explicitly re-export key items for easier consumption
pub use mentors::mentors_router;
pub use mentors::MentorsService;
pub use mentors::MentorsRepository;
pub use mentors::MentorSchema;
