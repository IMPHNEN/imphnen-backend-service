use axum::Router;

pub mod mentors;

pub fn dimentorin_router() -> Router {
	Router::new().nest("/mentors", mentors::mentors_router())
}
