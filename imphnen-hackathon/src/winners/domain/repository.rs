use super::entity::WinnerData;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;

#[async_trait]
pub trait WinnerRepository: Send + Sync {
	async fn list_winners(&self) -> Result<Vec<WinnerData>, AppError>;
}
