use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct WinnerData {
	pub id: Uuid,
	pub team_id: Uuid,
	pub team_name: String,
	pub rank: i32,
	pub prize: Option<String>,
	pub announced_at: Option<DateTime<Utc>>,
	pub created_at: Option<DateTime<Utc>>,
}
