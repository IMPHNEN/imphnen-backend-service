use crate::winners::domain::entity::WinnerData;
use crate::winners::domain::repository::WinnerRepository;
use crate::winners::domain::service::WinnerService;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use std::sync::Arc;

pub struct WinnerServiceImpl {
	repo: Arc<dyn WinnerRepository>,
}

impl WinnerServiceImpl {
	pub fn new(repo: Arc<dyn WinnerRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl WinnerService for WinnerServiceImpl {
	async fn list_winners(&self) -> Result<Vec<WinnerData>, AppError> {
		self.repo.list_winners().await
	}
}
