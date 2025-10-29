use axum::Router;

pub mod mentors;
pub mod sessions;

/// Creates the main Dimentorin router with all version 1 endpoints
/// Routes:
/// - /mentors -> mentors::mentors_router()
/// - /sessions -> sessions::sessions_router()
/// - /users/me/sessions -> sessions::get_my_sessions()
pub fn dimentorin_router() -> Router {
	Router::new()
		.nest("/mentors", mentors::mentors_router())
		.merge(sessions::sessions_router())
}

// Explicitly re-export key items for easier consumption
pub use mentors::mentors_router;
pub use mentors::MentorsService;
pub use mentors::MentorsRepository;
pub use mentors::MentorSchema;

pub use sessions::sessions_router;
pub use sessions::SessionsService;
pub use sessions::SessionsRepository;
pub use sessions::SessionSchema;
