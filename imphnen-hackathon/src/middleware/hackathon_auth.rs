use axum::http::StatusCode;
use axum::{
	body::Body,
	extract::Request,
	middleware::Next,
	response::{IntoResponse, Response},
};
use imphnen_libs::decode_access_token;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HackathonAuthUser {
	pub user_id: Uuid,
	pub is_admin: bool,
}

pub async fn hackathon_auth_middleware(
	axum::Extension(pool): axum::Extension<Arc<PgPool>>,
	mut request: Request<Body>,
	next: Next,
) -> Result<Response, Response> {
	let auth_header = request
		.headers()
		.get("Authorization")
		.and_then(|h| h.to_str().ok())
		.ok_or_else(|| {
			(StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response()
		})?;

	let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
		(
			StatusCode::UNAUTHORIZED,
			"Invalid Authorization header format",
		)
			.into_response()
	})?;

	let token_data = decode_access_token(token).map_err(|_| {
		(StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response()
	})?;

	let user_id = Uuid::parse_str(&token_data.claims.user_id).map_err(|_| {
		(StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response()
	})?;

	let is_admin: bool = sqlx::query_scalar(
		"SELECT COALESCE(is_admin, false) FROM hackathon_users WHERE id = $1",
	)
	.bind(user_id)
	.fetch_optional(pool.as_ref())
	.await
	.unwrap_or(None)
	.unwrap_or(false);

	request
		.extensions_mut()
		.insert(HackathonAuthUser { user_id, is_admin });
	Ok(next.run(request).await)
}
