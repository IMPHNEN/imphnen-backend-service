use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GachaCreditEntity {
	pub id: Uuid,
	pub user_id: Uuid,
	pub available_rolls: i32,
	pub is_deleted: bool,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}
