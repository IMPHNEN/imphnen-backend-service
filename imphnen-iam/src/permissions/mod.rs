pub mod application;
pub mod domain;
pub mod infrastructure;

pub use infrastructure::http::routes::{
	permissions_protected_routes, permissions_public_routes,
};
