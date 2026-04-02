use async_trait::async_trait;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;
use imphnen_utils::AppError;
use super::role::RoleEntity;

#[async_trait]
pub trait RoleService: Send + Sync {
    async fn list(&self, params: PaginationParams) -> Result<PaginatorResponse<RoleEntity>, AppError>;
    async fn get(&self, id: String) -> Result<RoleEntity, AppError>;
    async fn create(&self, name: String, permissions: Vec<String>) -> Result<RoleEntity, AppError>;
    async fn update(&self, id: String, name: Option<String>, permissions: Option<Vec<String>>) -> Result<String, AppError>;
    async fn delete(&self, id: String) -> Result<String, AppError>;
}
