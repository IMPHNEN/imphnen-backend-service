use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::gacha_roll::GachaRollEntity;

#[async_trait]
pub trait GachaRollService: Send + Sync {
    async fn get_roll(&self, id: Uuid) -> Result<GachaRollEntity, AppError>;
    async fn create_roll(&self, entity: GachaRollEntity) -> Result<(), AppError>;
    async fn execute_roll(&self, user_id: Uuid) -> Result<GachaRollEntity, AppError>;
    async fn delete_roll(&self, id: Uuid) -> Result<(), AppError>;
}
