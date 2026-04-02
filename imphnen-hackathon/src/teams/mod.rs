pub mod application;
pub mod domain;
pub mod infrastructure;

pub use infrastructure::http::routes::build_team_routes;
