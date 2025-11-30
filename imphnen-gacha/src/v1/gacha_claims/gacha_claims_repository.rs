use crate::v1::gacha_claims::gacha_claims_dto::GachaClaimQueryDto;
use crate::v1::gacha_claims::gacha_claims_schema::GachaClaimSchema;
use crate::v1::gacha_items::gacha_items_schema::GachaItemSchema;
use crate::AppState;
use anyhow::{Result, anyhow};
use imphnen_entities::seaorm::gacha::gacha_claims::{Entity as GachaClaimsEntity, ActiveModel as GachaClaimsActiveModel};
use imphnen_entities::seaorm::gacha::gacha_items::Entity as GachaItemsEntity;
use imphnen_iam::{UsersRepository, UsersDetailQueryDto};
use sea_orm::{EntityTrait, ActiveModelTrait, ActiveValue};
use tracing::instrument;
use uuid::Uuid;

pub struct GachaClaimRepository<'a> {
	state: &'a AppState,
}

impl<'a> GachaClaimRepository<'a> {
	pub fn new(state: &'a AppState) -> Self {
		Self { state }
	}

	#[instrument(skip(self, id), err)]
	pub async fn query_gacha_claim_by_id(
		&self,
		id: String,
	) -> Result<GachaClaimQueryDto> {
		let conn = &self.state.postgres_connection.conn;
		let claim_uuid = Uuid::parse_str(&id).map_err(|e| anyhow!("Invalid ID format: {}", e))?;

		let claim_model = GachaClaimsEntity::find_by_id(claim_uuid)
			.one(conn)
			.await?
			.ok_or_else(|| anyhow!("Gacha claim not found"))?;

		// Fetch User
		let user_repo = UsersRepository::new(self.state);
		let user_dto: UsersDetailQueryDto = user_repo
			.query_user_by_id(&claim_model.user_id.to_string())
			.await
			.map_err(|e| anyhow!("Failed to fetch user: {}", e))?;

		// Fetch Item
		let item_model = GachaItemsEntity::find_by_id(claim_model.gacha_item_id)
			.one(conn)
			.await?
			.ok_or_else(|| anyhow!("Gacha item not found"))?;
		
		// Convert Item Model to Schema
        let item_schema = GachaItemSchema {
            id: item_model.id.to_string(),
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
            image_url: "".to_string(), // Field not present in DB model
            is_deleted: item_model.deleted_at.is_some(),
            created_at: Some(item_model.created_at.to_rfc3339()),
            updated_at: Some(item_model.updated_at.to_rfc3339()),
        };

		Ok(GachaClaimQueryDto {
			id: claim_model.id.to_string(),
			user: user_dto,
			item: item_schema,
			is_deleted: claim_model.deleted_at.is_some(),
			created_at: Some(claim_model.created_at.to_rfc3339()),
			updated_at: Some(claim_model.updated_at.to_rfc3339()),
		})
	}

	#[instrument(skip(self, data), err)]
	pub async fn query_create_gacha_claim(
		&self,
		data: GachaClaimSchema,
	) -> Result<String> {
		let conn = &self.state.postgres_connection.conn;
        
        // Extract UUIDs from data (which uses Strings)
        // GachaClaimSchema uses "thing" format (e.g., "Users:uuid"), we might need to strip prefix if present,
        // but looking at schema implementation it seems it might store just UUID string or thing string.
        // Let's assume it's a UUID string or clean it.
        
        let user_id_str = data.user.split(':').next_back().unwrap_or(&data.user);
        let item_id_str = data.item.split(':').next_back().unwrap_or(&data.item);
        
		let user_uuid = Uuid::parse_str(user_id_str).map_err(|e| anyhow!("Invalid User UUID: {}", e))?;
		let item_uuid = Uuid::parse_str(item_id_str).map_err(|e| anyhow!("Invalid Item UUID: {}", e))?;
        let claim_id = Uuid::new_v4(); // Generate a new ID for the claim record itself
        let id_uuid = if data.id.is_empty() {
             Uuid::new_v4()
        } else {
             let clean_id = data.id.split(':').next_back().unwrap_or(&data.id);
             Uuid::parse_str(clean_id).unwrap_or_else(|_| Uuid::new_v4())
        };

		let active_model = GachaClaimsActiveModel {
			id: ActiveValue::Set(id_uuid),
			user_id: ActiveValue::Set(user_uuid),
			gacha_item_id: ActiveValue::Set(item_uuid),
			claim_id: ActiveValue::Set(claim_id), // Using random UUID for claim_id as it's required but not in Schema
			claim_type: ActiveValue::Set("standard".to_string()), // Default value
			status: ActiveValue::Set("claimed".to_string()), // Default value
			quantity: ActiveValue::Set(1),
			metadata: ActiveValue::Set(None),
			created_at: ActiveValue::Set(chrono::Utc::now()),
			updated_at: ActiveValue::Set(chrono::Utc::now()),
            deleted_at: ActiveValue::NotSet,
            claimed_at: ActiveValue::Set(chrono::Utc::now()),
		};

		let result = active_model.insert(conn).await?;

		Ok(result.id.to_string())
	}
}
