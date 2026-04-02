use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GachaItemEntity {
	pub id: Uuid,
	pub item_code: String,
	pub name: String,
	pub description: String,
	pub rarity: String,
	pub type_: String,
	pub category: String,
	pub value: i32,
	pub weight: f64,
	pub stock: i32,
	pub is_limited: bool,
	pub metadata: Option<Value>,
	pub is_deleted: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
	pub deleted_at: Option<DateTime<Utc>>,
}
