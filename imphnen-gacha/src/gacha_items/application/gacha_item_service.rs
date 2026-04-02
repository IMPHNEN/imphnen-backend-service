use std::sync::Arc;
use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use crate::gacha_items::domain::{GachaItemEntity, GachaItemRepository, GachaItemService};

pub struct GachaItemServiceImpl {
    repo: Arc<dyn GachaItemRepository>,
}

impl GachaItemServiceImpl {
    pub fn new(repo: Arc<dyn GachaItemRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl GachaItemService for GachaItemServiceImpl {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<GachaItemEntity>, AppError> {
        self.repo.find_all(params).await
    }

    async fn get(&self, id: Uuid) -> Result<GachaItemEntity, AppError> {
        self.repo.find_by_id(id).await
    }

    async fn create(&self, entity: GachaItemEntity) -> Result<(), AppError> {
        self.repo.create(entity).await
    }

    async fn update(&self, entity: GachaItemEntity) -> Result<(), AppError> {
        self.repo.update(entity).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
