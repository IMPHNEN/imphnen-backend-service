use crate::roadmap::domain::{RoadmapEntity, RoadmapRepository, RoadmapService};
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

pub struct RoadmapServiceImpl {
	repo: Arc<dyn RoadmapRepository>,
}

impl RoadmapServiceImpl {
	pub fn new(repo: Arc<dyn RoadmapRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl RoadmapService for RoadmapServiceImpl {
	async fn list(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoadmapEntity>, AppError> {
		self.repo.find_all(params).await
	}

	async fn get(&self, id: Uuid) -> Result<RoadmapEntity, AppError> {
		self.repo.find_by_id(id).await
	}

	async fn create(&self, entity: RoadmapEntity) -> Result<(), AppError> {
		self.repo.create(entity).await
	}

	async fn update(&self, entity: RoadmapEntity) -> Result<(), AppError> {
		self.repo.update(entity).await
	}

	async fn delete(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.delete(id).await
	}

	async fn vote(&self, id: Uuid) -> Result<(), AppError> {
		self.repo.increment_votes(id).await
	}
}
