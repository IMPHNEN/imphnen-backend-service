use super::gacha_claim::{GachaClaimDetail, GachaClaimEntity};
use async_trait::async_trait;
use imphnen_utils::AppError;
use uuid::Uuid;

#[async_trait]
pub trait GachaClaimRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> Result<GachaClaimDetail, AppError>;
	async fn create(&self, entity: GachaClaimEntity) -> Result<(), AppError>;
}
