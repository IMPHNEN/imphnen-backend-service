use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::{ActiveValue, QueryFilter};
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::gacha::gacha_rolls::{
    Entity as GachaRollsEntity, Column as GachaRollColumn,
    ActiveModel as GachaRollActiveModel, Model as GachaRollModel,
};
use crate::gacha_rolls::domain::{gacha_roll::GachaRollEntity, repository::GachaRollRepository};

fn to_entity(model: GachaRollModel) -> GachaRollEntity {
    GachaRollEntity {
        id: model.id,
        user_id: model.user_id,
        gacha_id: model.gacha_id,
        item_id: model.item_id,
        weight: model.weight,
        quantity: model.quantity,
        is_deleted: model.is_deleted,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

pub struct PostgresGachaRollRepository {
    db: Arc<DatabaseConnection>,
}

impl PostgresGachaRollRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl GachaRollRepository for PostgresGachaRollRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<GachaRollEntity, AppError> {
        let roll = GachaRollsEntity::find_by_id(id)
            .filter(GachaRollColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Gacha roll not found".to_string()))?;

        Ok(to_entity(roll))
    }

    async fn find_all_active(&self) -> Result<Vec<GachaRollEntity>, AppError> {
        let rolls = GachaRollsEntity::find()
            .filter(GachaRollColumn::IsDeleted.eq(false))
            .filter(GachaRollColumn::Quantity.gt(0))
            .all(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(rolls.into_iter().map(to_entity).collect())
    }

    async fn create(&self, entity: GachaRollEntity) -> Result<(), AppError> {
        let active_model = GachaRollActiveModel {
            id: ActiveValue::Set(entity.id),
            user_id: ActiveValue::Set(entity.user_id),
            gacha_id: ActiveValue::Set(entity.gacha_id),
            item_id: ActiveValue::Set(entity.item_id),
            weight: ActiveValue::Set(entity.weight),
            quantity: ActiveValue::Set(entity.quantity),
            is_deleted: ActiveValue::Set(false),
            created_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
            updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
        };

        GachaRollsEntity::insert(active_model)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut active_model: GachaRollActiveModel = GachaRollsEntity::find_by_id(id)
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("Gacha roll not found".to_string()))?
            .into();

        active_model.is_deleted = ActiveValue::Set(true);
        active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        active_model
            .update(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
