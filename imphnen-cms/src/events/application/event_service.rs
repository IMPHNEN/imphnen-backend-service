use std::sync::Arc;
use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use uuid::Uuid;
use imphnen_utils::AppError;
use crate::events::domain::{EventEntity, EventRepository, EventService};

pub struct EventServiceImpl {
    repo: Arc<dyn EventRepository>,
}

impl EventServiceImpl {
    pub fn new(repo: Arc<dyn EventRepository>) -> Self {
        Self { repo }
    }
}

#[async_trait]
impl EventService for EventServiceImpl {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<EventEntity>, AppError> {
        self.repo.find_all(params).await
    }

    async fn get(&self, id: Uuid) -> Result<EventEntity, AppError> {
        self.repo.find_by_id(id).await
    }

    async fn create(&self, entity: EventEntity) -> Result<(), AppError> {
        self.repo.create(entity).await
    }

    async fn update(&self, entity: EventEntity) -> Result<(), AppError> {
        self.repo.update(entity).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete(id).await
    }
}
