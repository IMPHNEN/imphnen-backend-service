use axum::{response::IntoResponse, routing::get, Extension, Router};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct WinnerResponse {
    pub id: Uuid,
    pub team_id: Uuid,
    pub team_name: String,
    pub rank: i32,
    pub prize: Option<String>,
    pub announced_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

async fn list_winners_handler(
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<axum::response::Response, AppError> {
    let rows: Vec<WinnerResponse> = sqlx::query_as(
        "SELECT w.id, w.team_id, t.name as team_name, w.rank, w.prize, w.announced_at, w.created_at FROM hackathon_winners w JOIN hackathon_teams t ON w.team_id = t.id ORDER BY w.rank ASC"
    )
    .fetch_all(pool.as_ref())
    .await
    .map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiSuccess(rows).into_response())
}

pub fn hackathon_winners_routes(pool: Arc<PgPool>) -> Router {
    Router::new()
        .route("/winners", get(list_winners_handler))
        .layer(Extension(pool))
}
