use super::gacha_credit::GachaCreditEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use uuid::Uuid;

#[async_trait]
pub trait GachaCreditService: Send + Sync {
	async fn get_credits(
		&self,
		user_id: Uuid,
	) -> Result<Option<GachaCreditEntity>, AppError>;
	async fn add_credits(&self, user_id: Uuid, amount: i32) -> Result<(), AppError>;
	async fn consume_credit(&self, user_id: Uuid) -> Result<(), AppError>;
}
