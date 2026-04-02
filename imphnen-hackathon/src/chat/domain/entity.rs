use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct MessageEntity {
	pub id: Uuid,
	pub team_id: Uuid,
	pub user_id: Uuid,
	pub message: String,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
pub struct MessageWithUser {
	pub id: Uuid,
	pub team_id: Uuid,
	pub user_id: Uuid,
	pub user_fullname: String,
	pub user_avatar: Option<String>,
	pub message: String,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Default)]
pub struct SendMessageInput {
	pub message: String,
}
