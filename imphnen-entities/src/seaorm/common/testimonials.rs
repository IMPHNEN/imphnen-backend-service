//! SeaORM entity for Testimonials table

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid; // Added Uuid import

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "testimonials")]
pub struct Model {
    #[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(not_null, column_type = "Uuid")]
    pub user_id: Uuid,

    #[sea_orm(not_null)]
    pub role: String,

    #[sea_orm(not_null)]
    pub content: String,

    #[sea_orm(default = "false")]
    pub is_deleted: bool,

    #[sea_orm(not_null, default = "now()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(not_null, default = "now()")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "crate::seaorm::auth::users::Entity",
        from = "Column::UserId",
        to = "crate::seaorm::auth::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
}

impl Related<crate::seaorm::auth::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}