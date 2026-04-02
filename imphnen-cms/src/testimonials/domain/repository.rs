use super::testimonial::TestimonialEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;

#[async_trait]
pub trait TestimonialRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<TestimonialEntity>, AppError>;
	async fn find_by_id(&self, id: Uuid) -> Result<TestimonialEntity, AppError>;
	async fn create(
		&self,
		entity: TestimonialEntity,
	) -> Result<TestimonialEntity, AppError>;
	async fn update(&self, entity: TestimonialEntity) -> Result<(), AppError>;
	async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
