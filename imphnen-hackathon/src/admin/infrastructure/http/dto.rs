use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PageQuery {
	#[serde(default = "default_page")]
	pub page: i64,
	#[serde(default = "default_limit")]
	pub limit: i64,
	pub search: Option<String>,
	pub status: Option<String>,
}

fn default_page() -> i64 {
	1
}
fn default_limit() -> i64 {
	20
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PagedResponse<T> {
	pub data: Vec<T>,
	pub total: i64,
	pub page: i64,
	pub limit: i64,
}

#[derive(Deserialize, ToSchema)]
pub struct SetAdminRequest {
	pub is_admin: bool,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct SetWinnerRequest {
	pub team_id: Uuid,
	pub rank: i32,
	pub prize: Option<String>,
}
