use async_trait::async_trait;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::gacha_credit::GachaCreditEntity;

#[async_trait]
pub trait GachaCreditService: Send + Sync {
    async fn get_credits(&self, user_id: Uuid) -> Result<Option<GachaCreditEntity>, AppError>;
    async fn add_credits(&self, user_id: Uuid, amount: i32) -> Result<(), AppError>;
    async fn consume_credit(&self, user_id: Uuid) -> Result<(), AppError>;
}
