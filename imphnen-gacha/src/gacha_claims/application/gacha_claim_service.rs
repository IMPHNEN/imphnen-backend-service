use crate::gacha_claims::domain::{
	GachaClaimDetail, GachaClaimEntity, GachaClaimRepository, GachaClaimService,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct GachaClaimServiceImpl {
	repo: Arc<dyn GachaClaimRepository>,
}

impl GachaClaimServiceImpl {
	pub fn new(repo: Arc<dyn GachaClaimRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl GachaClaimService for GachaClaimServiceImpl {
	async fn get_claim(&self, id: Uuid) -> Result<GachaClaimDetail, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn create_claim(&self, entity: GachaClaimEntity) -> Result<(), AppError> {
		self.repo.create(entity).await
	}
}
