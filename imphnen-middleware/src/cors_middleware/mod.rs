use axum::http::{HeaderValue, Method, header};
use imphnen_libs::enviroment::ENV;
use tower_http::cors::CorsLayer;

pub fn cors_middleware() -> CorsLayer {
	let env = &ENV;
	let cors_origins = match env.rust_env.as_str() {
		"development" => vec!["http://localhost:3000"],
		"production" => {
			vec![
				"https://gacha.imphnen.dev",
				"https://imphnen.dev",
				"https://dimentorin.imphnen.dev",
			]
		}
		_ => vec![
			"http://localhost:3000",
			"https://gacha.imphnen.dev",
			"https://imphnen.dev",
			"https://dimentorin.imphnen.dev",
		],
	};
	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true)
}
