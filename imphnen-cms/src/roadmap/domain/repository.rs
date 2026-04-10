use super::roadmap::RoadmapEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;

#[async_trait]
pub trait RoadmapRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoadmapEntity>, AppError>;
	async fn find_by_id(&self, id: Uuid) -> Result<RoadmapEntity, AppError>;
	async fn create(&self, entity: RoadmapEntity) -> Result<(), AppError>;
	async fn update(&self, entity: RoadmapEntity) -> Result<(), AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
	async fn increment_votes(&self, id: Uuid) -> Result<(), AppError>;
}
