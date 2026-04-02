pub mod application;
pub mod domain;
pub mod infrastructure;

pub use infrastructure::http::routes::{
	mentors_protected_routes, mentors_public_routes,
};
