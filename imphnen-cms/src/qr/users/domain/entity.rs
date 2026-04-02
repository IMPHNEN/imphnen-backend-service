use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserEntity {
	pub id: Uuid,
	pub email: String,
	pub name: String,
	pub role: String,
	pub provider: String,
	pub created_at: Option<DateTime<Utc>>,
	pub updated_at: Option<DateTime<Utc>>,
}

pub struct UpdateUserInput {
	pub name: Option<String>,
	pub email: Option<String>,
}
