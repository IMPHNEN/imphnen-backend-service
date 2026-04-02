use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "sessions")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(column_type = "Uuid")]
	pub mentor_id: Uuid,

	#[sea_orm(column_type = "Uuid")]
	pub mentee_id: Uuid,

	pub topic: String,

	#[sea_orm(nullable)]
	pub description: Option<String>,

	pub scheduled_at: DateTime<Utc>,

	pub duration_minutes: i32,

	#[sea_orm(nullable)]
	pub meeting_link: Option<String>,

	pub session_type: String, // "video_call", "phone_call", "chat"

	pub status: String, // "pending", "confirmed", "completed", "cancelled", "no_show"

	#[sea_orm(nullable)]
	pub feedback: Option<String>,

	#[sea_orm(nullable)]
	pub rating: Option<i32>, // 1-5

	#[sea_orm(nullable)]
	pub feedback_submitted_at: Option<DateTime<Utc>>,

	pub created_at: DateTime<Utc>,

	pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::users::Entity",
		from = "Column::MentorId",
		to = "super::users::Column::Id"
	)]
	Mentor,

	#[sea_orm(
		belongs_to = "super::users::Entity",
		from = "Column::MenteeId",
		to = "super::users::Column::Id"
	)]
	Mentee,
}

impl Related<super::users::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Mentor.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
