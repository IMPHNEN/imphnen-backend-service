use chrono::NaiveDateTime;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct GachaRollEntity {
	pub id: Uuid,
	pub user_id: Uuid,
	pub gacha_id: String,
	pub item_id: Uuid,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<NaiveDateTime>,
	pub updated_at: Option<NaiveDateTime>,
}
