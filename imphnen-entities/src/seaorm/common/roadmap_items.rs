use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "roadmap_items")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,

	#[sea_orm(not_null)]
	pub title: String,

	#[sea_orm(not_null)]
	pub description: String,

	#[sea_orm(not_null, default = "'upcoming'")]
	pub status: String,

	#[sea_orm(not_null, default = "0")]
	pub votes: i32,

	#[sea_orm(default = "false")]
	pub is_deleted: bool,

	#[sea_orm(not_null, default = "now()")]
	pub created_at: DateTime<Utc>,

	#[sea_orm(not_null, default = "now()")]
	pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
