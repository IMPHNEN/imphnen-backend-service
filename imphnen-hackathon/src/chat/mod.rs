pub mod domain;
pub mod application;
pub mod infrastructure;

pub use infrastructure::http::routes::build_chat_routes;
