//! SeaORM Entity for AuditLog

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, DeriveEntityModel)]
#[sea_orm(table_name = "app_audit_log")]
pub struct Model {
    #[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_email: String,
    pub action: String,
    pub resource: String,
    pub resource_id: Option<String>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub old_data: Option<Json>,
    #[sea_orm(column_type = "JsonBinary", nullable)]
    pub new_data: Option<Json>,
    pub ip_address: String,
    pub user_agent: Option<String>,
    pub timestamp: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
