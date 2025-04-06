use axum::{
	Extension,
	extract::Request,
	http::{HeaderValue, Method, StatusCode, header},
	middleware::Next,
	response::Response,
};
use imphnen_iam_service::{UsersItemDtoRaw, UsersRepository};
use imphnen_libs::{AppState, Env};
use imphnen_utils::{common_response, extract_email};
use std::convert::Infallible;
use tower_http::cors::CorsLayer;

pub async fn auth_middleware(
	Extension(state): Extension<AppState>,
	mut req: Request,
	next: Next,
) -> Result<Response, Infallible> {
	let headers = req.headers();
	let email = match extract_email(headers) {
		Some(email) => email,
		None => {
			return Ok(common_response(
				StatusCode::UNAUTHORIZED,
				"Invalid or expired token",
			));
		}
	};
	let repository = UsersRepository::new(&state);
	let user: Option<UsersItemDtoRaw> =
		match repository.query_user_by_email(email).await {
			Ok(user) => Some(user),
			Err(err) => {
				return Ok(common_response(
					StatusCode::INTERNAL_SERVER_ERROR,
					&err.to_string(),
				));
			}
		};
	if user.is_none() {
		return Ok(common_response(
			StatusCode::UNAUTHORIZED,
			"Unauthorized user",
		));
	}
	req.extensions_mut().insert(user.unwrap());
	Ok(next.run(req).await)
}

pub fn cors_middleware() -> CorsLayer {
	let env = Env::new();
	let cors_origins = match env.rust_env.as_str() {
		"development" => vec!["http://localhost:3000"],
		"production" => {
			vec!["https://gacha.imphnen.dev", "https://imphnen.dev"]
		}
		_ => vec![
			"http://localhost:3000",
			"https://gacha.imphnen.dev",
			"https://imphnen.dev",
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
