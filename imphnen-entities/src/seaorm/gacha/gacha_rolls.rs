use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Added Uuid import

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "gacha_rolls")]
pub struct Model {
	#[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
	pub id: Uuid,
	#[sea_orm(column_type = "Uuid")]
	pub user_id: Uuid,
	pub gacha_id: String,
	#[sea_orm(column_type = "Uuid")]
	pub item_id: Uuid,
	pub weight: f32,
	pub quantity: i32,
	pub is_deleted: bool,
	pub created_at: Option<DateTime>,
	pub updated_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
	#[sea_orm(
		belongs_to = "super::gacha_items::Entity",
		from = "Column::ItemId",
		to = "super::gacha_items::Column::Id"
	)]
	GachaItems,
	#[sea_orm(
		belongs_to = "super::super::auth::users::Entity",
		from = "Column::UserId",
		to = "super::super::auth::users::Column::Id"
	)]
	Users,
}

impl Related<super::gacha_items::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::GachaItems.def()
	}
}

impl Related<super::super::auth::users::Entity> for Entity {
	fn to() -> RelationDef {
		Relation::Users.def()
	}
}

impl ActiveModelBehavior for ActiveModel {}
