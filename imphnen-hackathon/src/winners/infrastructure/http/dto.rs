use crate::winners::domain::entity::WinnerData;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, ToSchema)]
pub struct WinnerResponse {
	pub id: Uuid,
	pub team_id: Uuid,
	pub team_name: String,
	pub rank: i32,
	pub prize: Option<String>,
	pub announced_at: Option<DateTime<Utc>>,
	pub created_at: Option<DateTime<Utc>>,
}

impl From<WinnerData> for WinnerResponse {
	fn from(d: WinnerData) -> Self {
		Self {
			id: d.id,
			team_id: d.team_id,
			team_name: d.team_name,
			rank: d.rank,
			prize: d.prize,
			announced_at: d.announced_at,
			created_at: d.created_at,
		}
	}
}
