use crate::gacha_claims::domain::{
	gacha_claim::{GachaClaimDetail, GachaClaimEntity},
	repository::GachaClaimRepository,
};
use crate::gacha_items::domain::gacha_item::GachaItemEntity;
use async_trait::async_trait;
use imphnen_entities::seaorm::gacha::gacha_claims::{
	ActiveModel as GachaClaimsActiveModel, Entity as GachaClaimsEntity,
};
use imphnen_entities::seaorm::gacha::gacha_items::Entity as GachaItemsEntity;
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use sea_orm::ActiveValue;
use sea_orm::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

pub struct PostgresGachaClaimRepository {
	db: Arc<DatabaseConnection>,
	state: Arc<AppState>,
}

impl PostgresGachaClaimRepository {
	pub fn new(db: DatabaseConnection, state: Arc<AppState>) -> Self {
		Self {
			db: Arc::new(db),
			state,
		}
	}
}

#[async_trait]
impl GachaClaimRepository for PostgresGachaClaimRepository {
	async fn find_by_id(&self, id: Uuid) -> Result<GachaClaimDetail, AppError> {
		let claim = GachaClaimsEntity::find_by_id(id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Gacha claim not found".to_string()))?;

		let user = self
			.state
			.user_lookup_service
			.get_user_by_id(claim.user_id, self.state.as_ref())
			.await
			.map(|info| info.basic_info)
			.map_err(|e| {
				AppError::InternalServerError(format!("Failed to fetch user: {e}"))
			})?;

		let item_model = GachaItemsEntity::find_by_id(claim.gacha_item_id)
			.one(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?
			.ok_or_else(|| AppError::NotFoundError("Gacha item not found".to_string()))?;

		let item = GachaItemEntity {
			id: item_model.id,
			item_code: item_model.item_code,
			name: item_model.name,
			description: item_model.description,
			rarity: item_model.rarity,
			type_: item_model.type_,
			category: item_model.category,
			value: item_model.value,
			weight: item_model.weight,
			stock: item_model.stock,
			is_limited: item_model.is_limited,
			metadata: item_model.metadata,
			is_deleted: item_model.deleted_at.is_some(),
			created_at: item_model.created_at,
			updated_at: item_model.updated_at,
			deleted_at: item_model.deleted_at,
		};

		Ok(GachaClaimDetail {
			id: claim.id,
			user,
			item,
			is_deleted: claim.deleted_at.is_some(),
			created_at: claim.created_at,
			updated_at: claim.updated_at,
		})
	}

	async fn create(&self, entity: GachaClaimEntity) -> Result<(), AppError> {
		let active_model = GachaClaimsActiveModel {
			id: ActiveValue::Set(entity.id),
			user_id: ActiveValue::Set(entity.user_id),
			gacha_item_id: ActiveValue::Set(entity.gacha_item_id),
			claim_id: ActiveValue::Set(entity.claim_id),
			claim_type: ActiveValue::Set(entity.claim_type),
			status: ActiveValue::Set(entity.status),
			quantity: ActiveValue::Set(entity.quantity),
			metadata: ActiveValue::Set(entity.metadata),
			created_at: ActiveValue::Set(entity.created_at),
			updated_at: ActiveValue::Set(entity.updated_at),
			deleted_at: ActiveValue::Set(entity.deleted_at),
			claimed_at: ActiveValue::Set(entity.claimed_at),
		};

		GachaClaimsEntity::insert(active_model)
			.exec(self.db.as_ref())
			.await
			.map_err(|e| AppError::InternalServerError(e.to_string()))?;

		Ok(())
	}
}
