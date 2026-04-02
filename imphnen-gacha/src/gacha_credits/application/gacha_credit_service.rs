use crate::gacha_credits::domain::{
	GachaCreditEntity, GachaCreditRepository, GachaCreditService,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct GachaCreditServiceImpl {
	repo: Arc<dyn GachaCreditRepository>,
}

impl GachaCreditServiceImpl {
	pub fn new(repo: Arc<dyn GachaCreditRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl GachaCreditService for GachaCreditServiceImpl {
	async fn get_credits(
		&self,
		user_id: Uuid,
	) -> Result<Option<GachaCreditEntity>, AppError> {
		self.repo.find_by_user_id(user_id).await
	}

	async fn add_credits(&self, user_id: Uuid, amount: i32) -> Result<(), AppError> {
		self.repo.add_credit(user_id, amount).await
	}

	async fn consume_credit(&self, user_id: Uuid) -> Result<(), AppError> {
		self.repo.consume_credit(user_id).await
	}
}
