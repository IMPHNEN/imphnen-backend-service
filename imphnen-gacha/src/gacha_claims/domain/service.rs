use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::gacha_claim::{GachaClaimDetail, GachaClaimEntity};

#[async_trait]
pub trait GachaClaimService: Send + Sync {
    async fn get_claim(&self, id: Uuid) -> Result<GachaClaimDetail, AppError>;
    async fn create_claim(&self, entity: GachaClaimEntity) -> Result<(), AppError>;
}
