//! SeaORM entity for RolesPermissions table
//! Corresponding to ResourceEnum::RolesPermissions
//! Represents the many-to-many relationship between Users and Roles

use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::{ActiveModelBehavior, DeriveEntityModel, DeriveRelation};
use sea_orm::ActiveValue::Set;
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "app_roles_permissions")]
pub struct Model {
    #[sea_orm(primary_key, default = "gen_random_uuid()", auto_increment = false)]
    pub id: Uuid,

    #[sea_orm(not_null)]
    pub user_id: Uuid,

    #[sea_orm(not_null)]
    pub role_id: Uuid,

    #[sea_orm(not_null)]
    pub permission_id: Uuid,

    #[sea_orm(not_null, default = "now()")]
    pub assigned_at: DateTime<Utc>,

    #[sea_orm(not_null, default = "false")]
    pub is_active: bool,

    #[sea_orm(not_null, default = "now()")]
    pub created_at: DateTime<Utc>,

    #[sea_orm(not_null, default = "now()")]
    pub updated_at: DateTime<Utc>,

    #[sea_orm(nullable)]
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, sea_orm::EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(belongs_to = "super::users::Entity", from = "Column::UserId", to = "super::users::Column::Id")]
    User,
    #[sea_orm(belongs_to = "super::roles::Entity", from = "Column::RoleId", to = "super::roles::Column::Id")]
    Role,
    #[sea_orm(belongs_to = "super::permissions::Entity", from = "Column::PermissionId", to = "super::permissions::Column::Id")]
    Permission,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl Related<super::roles::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Role.def()
    }
}

impl Related<super::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}

impl ActiveModelBehavior for ActiveModel {
    // Default implementation - SeaORM will handle timestamps automatically
}

// Builder pattern for RolePermission creation
#[derive(Default, Serialize, Deserialize)]
pub struct RolePermissionBuilder {
    user_id: Option<Uuid>,
    role_id: Option<Uuid>,
    permission_id: Option<Uuid>,
    is_active: Option<bool>,
}

impl RolePermissionBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn user_id(mut self, user_id: Uuid) -> Self {
        self.user_id = Some(user_id);
        self
    }

    #[must_use]
    pub fn role_id(mut self, role_id: Uuid) -> Self {
        self.role_id = Some(role_id);
        self
    }

    #[must_use]
    pub fn permission_id(mut self, permission_id: Uuid) -> Self {
        self.permission_id = Some(permission_id);
        self
    }

    #[must_use]
    pub fn is_active(mut self, is_active: bool) -> Self {
        self.is_active = Some(is_active);
        self
    }

    pub fn build(self) -> Result<ActiveModel, String> {
        let mut active_model = <ActiveModel as std::default::Default>::default();

        if let (Some(user_id), Some(role_id), Some(permission_id)) = (self.user_id, self.role_id, self.permission_id) {
            active_model.user_id = Set(user_id);
            active_model.role_id = Set(role_id);
            active_model.permission_id = Set(permission_id);
        } else {
            return Err("User ID, Role ID, and Permission ID are required".to_string());
        }

        if let Some(is_active) = self.is_active {
            active_model.is_active = Set(is_active);
        }

        Ok(active_model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seaorm::common::utils::generate_uuid;

    #[test]
    fn test_role_permission_model_creation() {
        let user_id = generate_uuid();
        let role_id = generate_uuid();
        let permission_id = generate_uuid();
        
        let role_permission = RolePermissionBuilder::new()
            .user_id(user_id)
            .role_id(role_id)
            .permission_id(permission_id)
            .is_active(true)
            .build();

        assert!(role_permission.is_ok());
        let role_permission_model = role_permission.unwrap();
        assert_eq!(role_permission_model.user_id, Set(user_id));
        assert_eq!(role_permission_model.role_id, Set(role_id));
        assert_eq!(role_permission_model.permission_id, Set(permission_id));
        assert_eq!(role_permission_model.is_active, Set(true));
    }
}