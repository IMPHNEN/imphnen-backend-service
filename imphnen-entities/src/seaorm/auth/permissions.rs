//! SeaORM entity for Permissions table
//! Corresponding to ResourceEnum::Permissions
//! Represents system permissions

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_permissions")]
pub struct Model {
    #[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(not_null)]
    pub name: String,

    #[sea_orm(not_null, default = "false")]
    pub is_deleted: bool,

    #[sea_orm(not_null, default = "now()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(not_null, default = "now()")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::roles_permissions::Entity")]
    RolesPermissions,
}

impl Related<super::roles_permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::RolesPermissions.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: Uuid) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_name(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Name.eq(name))
    }

    pub fn find_active() -> Select<Entity> {
        Self::find().filter(Column::IsDeleted.eq(false))
    }
}