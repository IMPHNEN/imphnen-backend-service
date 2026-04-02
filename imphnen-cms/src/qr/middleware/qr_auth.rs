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
pub struct QrAuthUser {
	pub user_id: Uuid,
	pub role: String,
}

pub async fn qr_auth_middleware(
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

	let _ = sqlx::query(
        "INSERT INTO qr_users (id, email, name, role, provider) VALUES ($1, $2, $2, 'user', 'external') ON CONFLICT (id) DO NOTHING"
    )
    .bind(user_id)
    .bind(&token_data.claims.sub)
    .execute(pool.as_ref())
    .await;

	let role: String = sqlx::query_scalar("SELECT role FROM qr_users WHERE id = $1")
		.bind(user_id)
		.fetch_optional(pool.as_ref())
		.await
		.ok()
		.flatten()
		.unwrap_or_else(|| "user".to_string());

	request
		.extensions_mut()
		.insert(QrAuthUser { user_id, role });
	Ok(next.run(request).await)
}
