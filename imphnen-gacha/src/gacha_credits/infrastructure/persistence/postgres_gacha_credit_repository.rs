use std::sync::Arc;
use async_trait::async_trait;
use sea_orm::prelude::*;
use sea_orm::ActiveValue;
use uuid::Uuid;
use imphnen_utils::AppError;
use imphnen_entities::seaorm::gacha::gacha_credits::{
    self, Entity as GachaCreditsEntity, Column as GachaCreditsColumn,
    ActiveModel as GachaCreditsActiveModel,
};
use crate::gacha_credits::domain::{gacha_credit::GachaCreditEntity, repository::GachaCreditRepository};

fn to_entity(model: gacha_credits::Model) -> GachaCreditEntity {
    GachaCreditEntity {
        id: model.id,
        user_id: model.user_id,
        available_rolls: model.available_rolls,
        is_deleted: model.is_deleted,
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}

pub struct PostgresGachaCreditRepository {
    db: Arc<DatabaseConnection>,
}

impl PostgresGachaCreditRepository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl GachaCreditRepository for PostgresGachaCreditRepository {
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<GachaCreditEntity>, AppError> {
        let result = GachaCreditsEntity::find()
            .filter(GachaCreditsColumn::UserId.eq(user_id))
            .filter(GachaCreditsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(result.map(to_entity))
    }

    async fn add_credit(&self, user_id: Uuid, amount: i32) -> Result<(), AppError> {
        let existing = GachaCreditsEntity::find()
            .filter(GachaCreditsColumn::UserId.eq(user_id))
            .filter(GachaCreditsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if let Some(credit) = existing {
            let mut active_model: GachaCreditsActiveModel = credit.clone().into();
            active_model.available_rolls = ActiveValue::Set(credit.available_rolls + amount);
            active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));
            GachaCreditsEntity::update(active_model)
                .exec(self.db.as_ref())
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        } else {
            let active_model = GachaCreditsActiveModel {
                id: ActiveValue::Set(Uuid::new_v4()),
                user_id: ActiveValue::Set(user_id),
                available_rolls: ActiveValue::Set(amount),
                is_deleted: ActiveValue::Set(false),
                created_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
                updated_at: ActiveValue::Set(Some(chrono::Utc::now().naive_utc())),
            };
            GachaCreditsEntity::insert(active_model)
                .exec(self.db.as_ref())
                .await
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        }

        Ok(())
    }

    async fn consume_credit(&self, user_id: Uuid) -> Result<(), AppError> {
        let credit = GachaCreditsEntity::find()
            .filter(GachaCreditsColumn::UserId.eq(user_id))
            .filter(GachaCreditsColumn::IsDeleted.eq(false))
            .one(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?
            .ok_or_else(|| AppError::NotFoundError("No credit record found".to_string()))?;

        if credit.available_rolls <= 0 {
            return Err(AppError::BadRequestError("No extra roll credits remaining".to_string()));
        }

        let mut active_model: GachaCreditsActiveModel = credit.clone().into();
        active_model.available_rolls = ActiveValue::Set(credit.available_rolls - 1);
        active_model.updated_at = ActiveValue::Set(Some(chrono::Utc::now().naive_utc()));

        GachaCreditsEntity::update(active_model)
            .exec(self.db.as_ref())
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        Ok(())
    }
}
