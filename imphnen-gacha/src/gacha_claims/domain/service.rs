use super::gacha_claim::{GachaClaimDetail, GachaClaimEntity};
use async_trait::async_trait;
use imphnen_utils::AppError;
use uuid::Uuid;

#[async_trait]
pub trait GachaClaimService: Send + Sync {
	async fn get_claim(&self, id: Uuid) -> Result<GachaClaimDetail, AppError>;
	async fn create_claim(&self, entity: GachaClaimEntity) -> Result<(), AppError>;
}
