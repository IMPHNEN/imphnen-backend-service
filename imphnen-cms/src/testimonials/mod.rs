pub mod application;
pub mod domain;
pub mod infrastructure;

pub use infrastructure::http::{
	testimonials_protected_routes, testimonials_public_routes,
};
