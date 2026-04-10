use super::roadmap::RoadmapEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;

#[async_trait]
pub trait RoadmapService: Send + Sync {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoadmapEntity>, AppError>;
	async fn get(&self, id: Uuid) -> Result<RoadmapEntity, AppError>;
	async fn create(&self, entity: RoadmapEntity) -> Result<(), AppError>;
	async fn update(&self, entity: RoadmapEntity) -> Result<(), AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
	async fn vote(&self, id: Uuid) -> Result<(), AppError>;
}
