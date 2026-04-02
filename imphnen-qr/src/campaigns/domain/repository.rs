use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

use super::entity::{CampaignEntity, CreateCampaignInput};

#[async_trait]
pub trait CampaignRepository: Send + Sync {
    async fn create(&self, input: CreateCampaignInput) -> Result<CampaignEntity, AppError>;
    async fn find_all(&self) -> Result<Vec<CampaignEntity>, AppError>;
    async fn find_active_qr_data(&self) -> Result<Option<Vec<u8>>, AppError>;
    async fn set_active(&self, id: Uuid) -> Result<CampaignEntity, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
