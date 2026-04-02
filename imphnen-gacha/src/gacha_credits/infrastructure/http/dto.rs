use imphnen_libs::ZodValidate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::gacha_credits::domain::gacha_credit::GachaCreditEntity;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreditAddRequestDto {
    pub amount: i32,
}

impl ZodValidate for GachaCreditAddRequestDto {
    fn zod_validate(value: &serde_json::Value) -> Result<Self, String> {
        serde_json::from_value(value.clone()).map_err(|e| e.to_string())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct GachaCreditDto {
    pub id: String,
    pub user_id: String,
    pub available_rolls: i32,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

impl From<GachaCreditEntity> for GachaCreditDto {
    fn from(e: GachaCreditEntity) -> Self {
        GachaCreditDto {
            id: e.id.to_string(),
            user_id: e.user_id.to_string(),
            available_rolls: e.available_rolls,
            is_deleted: e.is_deleted,
            created_at: e.created_at.map(|d| d.to_string()),
            updated_at: e.updated_at.map(|d| d.to_string()),
        }
    }
}
