use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use super::testimonial::TestimonialEntity;

#[async_trait]
pub trait TestimonialService: Send + Sync {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<TestimonialEntity>, AppError>;
    async fn get(&self, id: Uuid) -> Result<TestimonialEntity, AppError>;
    async fn create(&self, entity: TestimonialEntity) -> Result<TestimonialEntity, AppError>;
    async fn update(&self, entity: TestimonialEntity) -> Result<(), AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
