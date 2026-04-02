use axum::{
    extract::{Path, Query},
    middleware::from_fn,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};
use sqlx::{PgPool, FromRow};
use std::sync::Arc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use imphnen_utils::{errors::AppError, response_format::{ApiSuccess, ApiMessage}};
use crate::middleware::{admin_only::admin_only, hackathon_auth::hackathon_auth_middleware};
use crate::common::hackathon_jwt::HackathonJwtService;

#[derive(Deserialize)]
struct PageQuery {
    #[serde(default = "default_page")]
    page: i64,
    #[serde(default = "default_limit")]
    limit: i64,
    search: Option<String>,
    status: Option<String>,
}
fn default_page() -> i64 { 1 }
fn default_limit() -> i64 { 20 }

#[derive(Debug, Serialize, ToSchema, FromRow)]
struct AdminUserRow {
    id: Uuid,
    email: String,
    fullname: String,
    avatar: Option<String>,
    is_active: Option<bool>,
    is_admin: Option<bool>,
    created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, ToSchema)]
struct PagedResponse<T> {
    data: Vec<T>,
    total: i64,
    page: i64,
    limit: i64,
}

#[derive(Deserialize, ToSchema)]
struct SetAdminRequest { is_admin: bool }

async fn admin_list_users(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
    let offset = (q.page - 1) * q.limit;
    let pattern = q.search.as_deref().map(|s| format!("%{}%", s));
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_users WHERE ($1::text IS NULL OR email ILIKE $1 OR fullname ILIKE $1)")
        .bind(&pattern).fetch_one(pool.as_ref()).await.unwrap_or(0);
    let users: Vec<AdminUserRow> = sqlx::query_as("SELECT id, email, fullname, avatar, is_active, is_admin, created_at FROM hackathon_users WHERE ($1::text IS NULL OR email ILIKE $1 OR fullname ILIKE $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(&pattern).bind(q.limit).bind(offset)
        .fetch_all(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiSuccess(PagedResponse { data: users, total, page: q.page, limit: q.limit }).into_response())
}

async fn admin_get_user(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
    let user: AdminUserRow = sqlx::query_as("SELECT id, email, fullname, avatar, is_active, is_admin, created_at FROM hackathon_users WHERE id = $1")
        .bind(user_id).fetch_optional(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?
        .ok_or_else(|| AppError::NotFoundError("User not found".to_string()))?;
    Ok(ApiSuccess(user).into_response())
}

async fn admin_set_admin(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
    Json(body): Json<SetAdminRequest>,
) -> Result<ApiMessage, AppError> {
    sqlx::query("UPDATE hackathon_users SET is_admin = $1 WHERE id = $2")
        .bind(body.is_admin).bind(user_id)
        .execute(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiMessage::ok("User admin status updated"))
}

async fn admin_delete_user(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(user_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
    sqlx::query("DELETE FROM hackathon_users WHERE id = $1")
        .bind(user_id).execute(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiMessage::ok("User deleted"))
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
struct AdminTeamRow {
    id: Uuid, name: String, city: String, visibility: String,
    leader_id: Uuid, created_at: chrono::DateTime<chrono::Utc>,
}

async fn admin_list_teams(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
    let offset = (q.page - 1) * q.limit;
    let pattern = q.search.as_deref().map(|s| format!("%{}%", s));
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_teams WHERE ($1::text IS NULL OR name ILIKE $1)")
        .bind(&pattern).fetch_one(pool.as_ref()).await.unwrap_or(0);
    let teams: Vec<AdminTeamRow> = sqlx::query_as("SELECT id, name, city, visibility, leader_id, created_at FROM hackathon_teams WHERE ($1::text IS NULL OR name ILIKE $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(&pattern).bind(q.limit).bind(offset)
        .fetch_all(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiSuccess(PagedResponse { data: teams, total, page: q.page, limit: q.limit }).into_response())
}

async fn admin_delete_team(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
    sqlx::query("DELETE FROM hackathon_teams WHERE id = $1")
        .bind(team_id).execute(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiMessage::ok("Team deleted"))
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
struct AdminSubmissionRow {
    id: Uuid, team_id: Uuid, project_name: String, status: String,
    submitted_at: Option<chrono::DateTime<chrono::Utc>>, created_at: Option<chrono::DateTime<chrono::Utc>>,
}

async fn admin_list_submissions(
    Extension(pool): Extension<Arc<PgPool>>,
    Query(q): Query<PageQuery>,
) -> Result<axum::response::Response, AppError> {
    let offset = (q.page - 1) * q.limit;
    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM hackathon_project_submissions WHERE ($1::text IS NULL OR status = $1)")
        .bind(&q.status).fetch_one(pool.as_ref()).await.unwrap_or(0);
    let subs: Vec<AdminSubmissionRow> = sqlx::query_as("SELECT id, team_id, project_name, status, submitted_at, created_at FROM hackathon_project_submissions WHERE ($1::text IS NULL OR status = $1) ORDER BY created_at DESC LIMIT $2 OFFSET $3")
        .bind(&q.status).bind(q.limit).bind(offset)
        .fetch_all(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiSuccess(PagedResponse { data: subs, total, page: q.page, limit: q.limit }).into_response())
}

#[derive(Debug, Deserialize, ToSchema)]
struct SetWinnerRequest { team_id: Uuid, rank: i32, prize: Option<String> }

#[derive(Debug, Serialize, ToSchema, FromRow)]
struct WinnerRow { id: Uuid, team_id: Uuid, rank: i32, prize: Option<String>, created_at: Option<chrono::DateTime<chrono::Utc>> }

async fn admin_set_winner(
    Extension(pool): Extension<Arc<PgPool>>,
    Json(body): Json<SetWinnerRequest>,
) -> Result<ApiMessage, AppError> {
    sqlx::query("INSERT INTO hackathon_winners (id, team_id, rank, prize, announced_at, created_at, updated_at) VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW()) ON CONFLICT (team_id) DO UPDATE SET rank = $3, prize = $4, updated_at = NOW()")
        .bind(Uuid::new_v4()).bind(body.team_id).bind(body.rank).bind(body.prize)
        .execute(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiMessage::ok("Winner set"))
}

async fn admin_remove_winner(
    Extension(pool): Extension<Arc<PgPool>>,
    Path(team_id): Path<Uuid>,
) -> Result<ApiMessage, AppError> {
    sqlx::query("DELETE FROM hackathon_winners WHERE team_id = $1")
        .bind(team_id).execute(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiMessage::ok("Winner removed"))
}

async fn admin_list_winners(
    Extension(pool): Extension<Arc<PgPool>>,
) -> Result<axum::response::Response, AppError> {
    let rows: Vec<WinnerRow> = sqlx::query_as("SELECT id, team_id, rank, prize, created_at FROM hackathon_winners ORDER BY rank ASC")
        .fetch_all(pool.as_ref()).await.map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(ApiSuccess(rows).into_response())
}

pub fn hackathon_admin_routes(pool: Arc<PgPool>, jwt: Arc<HackathonJwtService>) -> Router {
    Router::new()
        .route("/admin/users", get(admin_list_users))
        .route("/admin/users/:user_id", get(admin_get_user).delete(admin_delete_user))
        .route("/admin/users/:user_id/set-admin", post(admin_set_admin))
        .route("/admin/teams", get(admin_list_teams))
        .route("/admin/teams/:team_id", delete(admin_delete_team))
        .route("/admin/submissions", get(admin_list_submissions))
        .route("/admin/winners", get(admin_list_winners).post(admin_set_winner))
        .route("/admin/winners/:team_id", delete(admin_remove_winner))
        .layer(Extension(pool.clone()))
        .layer(from_fn(admin_only))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(hackathon_auth_middleware))
}
