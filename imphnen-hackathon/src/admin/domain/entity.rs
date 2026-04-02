use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct AdminUserRow {
	pub id: Uuid,
	pub email: String,
	pub fullname: String,
	pub avatar: Option<String>,
	pub is_active: Option<bool>,
	pub is_admin: Option<bool>,
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct AdminTeamRow {
	pub id: Uuid,
	pub name: String,
	pub city: String,
	pub visibility: String,
	pub leader_id: Uuid,
	pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct AdminSubmissionRow {
	pub id: Uuid,
	pub team_id: Uuid,
	pub project_name: String,
	pub status: String,
	pub submitted_at: Option<chrono::DateTime<chrono::Utc>>,
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Serialize, ToSchema, FromRow)]
pub struct WinnerRow {
	pub id: Uuid,
	pub team_id: Uuid,
	pub rank: i32,
	pub prize: Option<String>,
	pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
