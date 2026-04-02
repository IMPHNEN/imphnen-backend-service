use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

use super::entity::CampaignEntity;

#[async_trait]
pub trait QrCampaignService: Send + Sync {
	async fn create(
		&self,
		name: String,
		url: String,
		created_by: Uuid,
	) -> Result<CampaignEntity, AppError>;
	async fn list_all(&self) -> Result<Vec<CampaignEntity>, AppError>;
	async fn get_active_qr_data(&self) -> Result<Option<Vec<u8>>, AppError>;
	async fn set_active(&self, id: Uuid) -> Result<CampaignEntity, AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
	async fn process_image(&self, image_bytes: Vec<u8>) -> Result<Vec<u8>, AppError>;
}
