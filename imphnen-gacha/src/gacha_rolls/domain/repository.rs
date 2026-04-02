use super::gacha_roll::GachaRollEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use uuid::Uuid;

#[async_trait]
pub trait GachaRollRepository: Send + Sync {
	async fn find_by_id(&self, id: Uuid) -> Result<GachaRollEntity, AppError>;
	async fn find_all_active(&self) -> Result<Vec<GachaRollEntity>, AppError>;
	async fn create(&self, entity: GachaRollEntity) -> Result<(), AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
