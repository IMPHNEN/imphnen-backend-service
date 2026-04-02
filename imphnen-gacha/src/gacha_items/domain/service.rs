use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::gacha_item::GachaItemEntity;

#[async_trait]
pub trait GachaItemService: Send + Sync {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<GachaItemEntity>, AppError>;
    async fn get(&self, id: Uuid) -> Result<GachaItemEntity, AppError>;
    async fn create(&self, entity: GachaItemEntity) -> Result<(), AppError>;
    async fn update(&self, entity: GachaItemEntity) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
