use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct RoadmapEntity {
	pub id: Uuid,
	pub title: String,
	pub description: String,
	pub status: String,
	pub votes: i32,
	pub is_deleted: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}
