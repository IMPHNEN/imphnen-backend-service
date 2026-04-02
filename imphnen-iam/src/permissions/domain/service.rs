use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use imphnen_utils::AppError;
use super::permission::PermissionEntity;

#[async_trait]
pub trait PermissionService: Send + Sync {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<PermissionEntity>, AppError>;
    async fn get(&self, id: String) -> Result<PermissionEntity, AppError>;
    async fn create(&self, name: String) -> Result<String, AppError>;
    async fn update(&self, entity: PermissionEntity) -> Result<String, AppError>;
    async fn delete(&self, id: String) -> Result<String, AppError>;
}
