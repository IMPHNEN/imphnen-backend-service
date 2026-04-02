use axum::{extract::Path, response::IntoResponse, routing::get, Extension, Router};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct CertificateResponse {
    pub user_id: Uuid,
    pub fullname: String,
    pub email: String,
    pub avatar: Option<String>,
    pub team_id: Option<Uuid>,
    pub team_name: Option<String>,
    pub is_leader: Option<bool>,
    pub project_name: Option<String>,
    pub submission_status: Option<String>,
    pub winner_rank: Option<i32>,
    pub winner_prize: Option<String>,
}

async fn get_certificate_handler(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let row: Option<CertificateResponse> = sqlx::query_as(
        "SELECT u.id as user_id, u.fullname, u.email, u.avatar, t.id as team_id, t.name as team_name, (t.leader_id = u.id) as is_leader, ps.project_name, ps.status as submission_status, w.rank as winner_rank, w.prize as winner_prize FROM hackathon_users u LEFT JOIN hackathon_team_members tm ON tm.user_id = u.id AND tm.status = 'active' LEFT JOIN hackathon_teams t ON t.id = tm.team_id LEFT JOIN hackathon_project_submissions ps ON ps.team_id = t.id LEFT JOIN hackathon_winners w ON w.team_id = t.id WHERE u.id = $1 LIMIT 1"
    )
    .bind(user_id)
    .fetch_optional(pool.as_ref())
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;

    let cert = row.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;
    Ok(ApiSuccess(cert).into_response())
}

pub fn hackathon_certificates_routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/certificates/:user_id", get(get_certificate_handler))
        .layer(Extension(pool))
}
