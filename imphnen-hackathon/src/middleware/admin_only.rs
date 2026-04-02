use axum::{
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::middleware::hackathon_auth::HackathonAuthUser;

pub async fn admin_only(
    Extension(auth_user): Extension<HackathonAuthUser>,
    req: Request<Body>,
    next: Next,
) -> Response {
    if !auth_user.is_admin {
        return (StatusCode::FORBIDDEN, Json(json!({ "message": "Forbidden - Admin access required" }))).into_response();
    }
    next.run(req).await
}
