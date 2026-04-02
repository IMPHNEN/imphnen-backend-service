use axum::{body::Body, extract::Request, middleware::Next, response::{IntoResponse, Response}};
use axum::http::StatusCode;
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::common::qr_jwt::QrJwtService;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QrAuthUser {
    pub user_id: Uuid,
    pub role: String,
}

pub async fn qr_auth_middleware(
    axum::Extension(jwt_service): axum::Extension<Arc<QrJwtService>>,
    mut request: Request<Body>,
    next: Next,
) -> Result<Response, Response> {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| (StatusCode::UNAUTHORIZED, "Missing Authorization header").into_response())?;

    let token = auth_header.strip_prefix("Bearer ").ok_or_else(|| {
        (StatusCode::UNAUTHORIZED, "Invalid Authorization header format").into_response()
    })?;

    let claims = jwt_service.verify_token(token).map_err(|_| {
        (StatusCode::UNAUTHORIZED, "Invalid or expired token").into_response()
    })?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        (StatusCode::UNAUTHORIZED, "Invalid user ID in token").into_response()
    })?;

    request.extensions_mut().insert(QrAuthUser { user_id, role: claims.role });
    Ok(next.run(request).await)
}
