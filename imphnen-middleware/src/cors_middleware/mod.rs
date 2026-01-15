use axum::http::{HeaderValue, Method, header};
use imphnen_libs::environment::ENV;
use tower_http::cors::CorsLayer;

pub fn cors_middleware() -> CorsLayer {
	let env = &ENV;
	let cors_origins = match env.rust_env.as_str() {
		"development" => {
			let mut origins = vec!["http://localhost:3000".to_string()];
			origins.push(format!("http://localhost:{}", env.port));
			origins
		},
		"production" => {
			vec![
				"https://gacha.imphnen.dev".to_string(),
				"https://imphnen.dev".to_string(),
				"https://dimentorin.imphnen.dev".to_string(),
			]
		}
		_ => vec![
			"http://localhost:3000".to_string(),
			"https://gacha.imphnen.dev".to_string(),
			"https://imphnen.dev".to_string(),
			"https://dimentorin.imphnen.dev".to_string(),
		],
	};
	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true)
}