use crate::testimonials::domain::{
	TestimonialEntity, TestimonialRepository, TestimonialService,
};
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

pub struct TestimonialServiceImpl {
	repo: Arc<dyn TestimonialRepository>,
}

impl TestimonialServiceImpl {
	pub fn new(repo: Arc<dyn TestimonialRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl TestimonialService for TestimonialServiceImpl {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<TestimonialEntity>, AppError> {
		self.repo.find_all(params).await
	}

	async fn get(&self, id: Uuid) -> Result<TestimonialEntity, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn create(
		&self,
		entity: TestimonialEntity,
	) -> Result<TestimonialEntity, AppError> {
		self.repo.create(entity).await
	}

	async fn update(&self, entity: TestimonialEntity) -> Result<(), AppError> {
		self.repo.update(entity).await
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.delete(id).await
	}
}
