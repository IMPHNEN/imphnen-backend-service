use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, QueryOrder, QuerySelect, PaginatorTrait};
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::auth::roles::{
    Entity as RolesEntity, Column as RolesColumn,
    ActiveModel as RolesActiveModel, Model as RolesModel,
};
use crate::roles::domain::{RoleEntity, RoleRepository};

fn to_entity(model: RolesModel) -> RoleEntity {
    let permissions = model.permissions.as_ref()
        .and_then(|p| serde_json::from_value::<Vec<String>>(p.clone()).ok())
        .unwrap_or_default();

    RoleEntity {
        id: model.id,
        name: model.name,
        description: model.description,
        is_system_role: model.is_system_role,
        is_default: model.is_default,
        permissions,
        created_at: Some(model.created_at.to_rfc3339()),
        updated_at: Some(model.updated_at.to_rfc3339()),
        deleted_at: model.deleted_at.map(|d| d.to_rfc3339()),
    }
}

pub struct PostgresRoleRepository {
    db: Arc<DatabaseConnection>,
}

impl PostgresRoleRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl RoleRepository for PostgresRoleRepository {
    async fn find_all(&self, params: PaginationParams) -> Result<PaginatorResponse<RoleEntity>, AppError> {
        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, 100);

        let mut query = RolesEntity::find()
            .filter(RolesColumn::DeletedAt.is_null());

        if let Some(ref search) = params.search {
            query = query.filter(RolesColumn::Name.contains(&search.query));
        }

        let sort_column = match params.sort_by.as_deref() {
            Some("name") => RolesColumn::Name,
            _ => RolesColumn::CreatedAt,
        };
        query = match params.sort_direction {
            Some(SortDirection::Desc) => query.order_by(sort_column, Order::Desc),
            _ => query.order_by(sort_column, Order::Asc),
        };

        let total_count = query.clone().count(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let offset = ((page - 1) * per_page) as u64;
        let roles = query.offset(offset).limit(per_page as u64).all(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let data = roles.into_iter().map(to_entity).collect();
        let meta = PaginatorResponseMeta::new(page, per_page, total_count as u32);
        Ok(PaginatorResponse { data, meta })
    }

    async fn find_by_id(&self, id: String) -> Result<RoleEntity, AppError> {
        let role_id = Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid role ID".into()))?;

        let model = RolesEntity::find_by_id(role_id)
            .filter(RolesColumn::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Role not found".into()))?;

        Ok(to_entity(model))
    }

    async fn find_by_name(&self, name: String) -> Result<RoleEntity, AppError> {
        let model = RolesEntity::find()
            .filter(RolesColumn::Name.eq(&name))
            .filter(RolesColumn::DeletedAt.is_null())
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Role not found".into()))?;

        Ok(to_entity(model))
    }

    async fn create(&self, entity: RoleEntity) -> Result<RoleEntity, AppError> {
        let permissions_json = serde_json::to_value(&entity.permissions)
            .map_err(|e| AppError::InternalServerError(format!("Failed to serialize permissions: {e}")))?;

        let active_model = RolesActiveModel {
            id: ActiveValue::Set(entity.id),
            name: ActiveValue::Set(entity.name),
            description: ActiveValue::Set(entity.description),
            is_system_role: ActiveValue::Set(entity.is_system_role),
            is_default: ActiveValue::Set(entity.is_default),
            permissions: ActiveValue::Set(Some(permissions_json)),
            created_at: ActiveValue::Set(chrono::Utc::now()),
            updated_at: ActiveValue::Set(chrono::Utc::now()),
            deleted_at: ActiveValue::NotSet,
        };

        let created = active_model.insert(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(to_entity(created))
    }

    async fn update(&self, id: String, name: Option<String>, permissions: Option<Vec<String>>) -> Result<String, AppError> {
        let role_id = Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid role ID".into()))?;

        let mut active_model = RolesActiveModel {
            id: ActiveValue::Unchanged(role_id),
            ..Default::default()
        };

        if let Some(n) = name {
            active_model.name = ActiveValue::Set(n);
        }
        if let Some(perms) = permissions {
            let permissions_json = serde_json::to_value(&perms)
                .map_err(|e| AppError::InternalServerError(format!("Failed to serialize permissions: {e}")))?;
            active_model.permissions = ActiveValue::Set(Some(permissions_json));
        }
        active_model.updated_at = ActiveValue::Set(chrono::Utc::now());

        active_model.update(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok("Success update role".into())
    }

    async fn delete(&self, id: String) -> Result<String, AppError> {
        let role_id = Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid role ID".into()))?;

        let active_model = RolesActiveModel {
            id: ActiveValue::Unchanged(role_id),
            deleted_at: ActiveValue::Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        active_model.update(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok("Success delete role".into())
    }
}
