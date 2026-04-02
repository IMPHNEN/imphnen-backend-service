use super::event::EventEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;

#[async_trait]
pub trait EventRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<EventEntity>, AppError>;
	async fn find_by_id(&self, id: Uuid) -> Result<EventEntity, AppError>;
	async fn create(&self, entity: EventEntity) -> Result<(), AppError>;
	async fn update(&self, entity: EventEntity) -> Result<(), AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
