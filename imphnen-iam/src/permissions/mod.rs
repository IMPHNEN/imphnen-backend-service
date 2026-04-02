pub mod domain;
pub mod application;
pub mod infrastructure;

pub use infrastructure::http::routes::{permissions_public_routes, permissions_protected_routes};
