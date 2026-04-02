use axum::http::{HeaderValue, Method, header};
use imphnen_libs::environment::ENV;
use tower_http::cors::CorsLayer;

pub fn cors_middleware() -> CorsLayer {
	let allowed_origins: Vec<HeaderValue> = ENV
		.cors_allowed_origins
		.iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([
			Method::GET,
			Method::POST,
			Method::PUT,
			Method::DELETE,
			Method::OPTIONS,
		])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true)
}
