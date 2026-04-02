use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::event::EventEntity;

#[async_trait]
pub trait EventService: Send + Sync {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<EventEntity>, AppError>;
    async fn get(&self, id: Uuid) -> Result<EventEntity, AppError>;
    async fn create(&self, entity: EventEntity) -> Result<(), AppError>;
    async fn update(&self, entity: EventEntity) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
