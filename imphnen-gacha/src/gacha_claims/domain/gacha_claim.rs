use chrono::{DateTime, Utc};
use serde_json::Value;
use uuid::Uuid;
use imphnen_entities::UsersDetailQueryDto;
use crate::gacha_items::domain::gacha_item::GachaItemEntity;

#[derive(Clone, Debug)]
pub struct GachaClaimEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub gacha_item_id: Uuid,
    pub claim_id: Uuid,
    pub claim_type: String,
    pub status: String,
    pub quantity: i32,
    pub metadata: Option<Value>,
    pub is_deleted: bool,
    pub claimed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

/// Denormalized struct for claim detail responses with nested user and item data.
#[derive(Clone, Debug)]
pub struct GachaClaimDetail {
    pub id: Uuid,
    pub user: UsersDetailQueryDto,
    pub item: GachaItemEntity,
    pub is_deleted: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
