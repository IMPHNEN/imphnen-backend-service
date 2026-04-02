use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, Order, QueryOrder, QuerySelect, PaginatorTrait};
use paginator_rs::{PaginationParams, SortDirection};
use paginator_utils::{PaginatorResponse, PaginatorResponseMeta};
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::auth::permissions::{
    Entity as PermissionsEntity, Column as PermissionsColumn,
    ActiveModel as PermissionsActiveModel, Model as PermissionsModel,
};
use crate::permissions::domain::{PermissionEntity, PermissionRepository};

fn to_entity(model: PermissionsModel) -> PermissionEntity {
    PermissionEntity {
        id: model.id,
        name: model.name,
        is_deleted: model.is_deleted,
        created_at: Some(model.created_at.to_rfc3339()),
        updated_at: Some(model.updated_at.to_rfc3339()),
    }
}

pub struct PostgresPermissionRepository {
    db: Arc<DatabaseConnection>,
}

impl PostgresPermissionRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl PermissionRepository for PostgresPermissionRepository {
    async fn find_all(&self, params: PaginationParams) -> Result<PaginatorResponse<PermissionEntity>, AppError> {
        let page = params.page.max(1);
        let per_page = params.per_page.clamp(1, 100);

        let mut query = PermissionsEntity::find()
            .filter(PermissionsColumn::IsDeleted.eq(false));

        if let Some(ref search) = params.search {
            query = query.filter(PermissionsColumn::Name.contains(&search.query));
        }

        let order_column = match params.sort_by.as_deref() {
            Some("name") => PermissionsColumn::Name,
            _ => PermissionsColumn::CreatedAt,
        };
        query = match params.sort_direction {
            Some(SortDirection::Desc) => query.order_by(order_column, Order::Desc),
            _ => query.order_by(order_column, Order::Asc),
        };

        let total_count = query.clone().count(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let offset = ((page - 1) * per_page) as u64;
        let permissions = query.offset(offset).limit(per_page as u64).all(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let data = permissions.into_iter().map(to_entity).collect();
        let meta = PaginatorResponseMeta::new(page, per_page, total_count as u32);
        Ok(PaginatorResponse { data, meta })
    }

    async fn find_by_id(&self, id: String) -> Result<PermissionEntity, AppError> {
        let perm_id = Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid permission ID".into()))?;

        let model = PermissionsEntity::find_by_id(perm_id)
            .filter(PermissionsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Permission not found".into()))?;

        Ok(to_entity(model))
    }

    async fn find_by_name(&self, name: String) -> Result<PermissionEntity, AppError> {
        let model = PermissionsEntity::find()
            .filter(PermissionsColumn::Name.eq(&name))
            .filter(PermissionsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Permission not found".into()))?;

        Ok(to_entity(model))
    }

    async fn create(&self, entity: PermissionEntity) -> Result<String, AppError> {
        let active_model = PermissionsActiveModel {
            id: ActiveValue::Set(entity.id),
            name: ActiveValue::Set(entity.name),
            is_deleted: ActiveValue::Set(false),
            created_at: ActiveValue::Set(chrono::Utc::now()),
            updated_at: ActiveValue::Set(chrono::Utc::now()),
            deleted_at: ActiveValue::NotSet,
        };

        let result = PermissionsEntity::insert(active_model)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(format!("Success create permission with id: {}", result.last_insert_id))
    }

    async fn update(&self, entity: PermissionEntity) -> Result<String, AppError> {
        let existing = PermissionsEntity::find_by_id(entity.id)
            .filter(PermissionsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Permission not found".into()))?;

        let active_model = PermissionsActiveModel {
            id: ActiveValue::Set(entity.id),
            name: ActiveValue::Set(entity.name),
            is_deleted: ActiveValue::Set(entity.is_deleted),
            created_at: ActiveValue::Unchanged(existing.created_at),
            updated_at: ActiveValue::Set(chrono::Utc::now()),
            deleted_at: ActiveValue::NotSet,
        };

        let result = active_model.update(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(format!("Success update permission with id: {}", result.id))
    }

    async fn delete(&self, id: String) -> Result<String, AppError> {
        let perm_id = Uuid::parse_str(&id)
            .map_err(|_| AppError::BadRequestError("Invalid permission ID".into()))?;

        let _existing = PermissionsEntity::find_by_id(perm_id)
            .filter(PermissionsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Permission not found".into()))?;

        let active_model = PermissionsActiveModel {
            id: ActiveValue::Set(perm_id),
            is_deleted: ActiveValue::Set(true),
            deleted_at: ActiveValue::Set(Some(chrono::Utc::now())),
            ..Default::default()
        };

        let result = active_model.update(self.db.as_ref()).await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(format!("Success delete permission with id: {}", result.id))
    }
}
