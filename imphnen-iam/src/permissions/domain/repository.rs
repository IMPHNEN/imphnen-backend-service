use super::permission::PermissionEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<PermissionEntity>, AppError>;
	async fn find_by_id(&self, id: String) -> Result<PermissionEntity, AppError>;
	async fn find_by_name(&self, name: String) -> Result<PermissionEntity, AppError>;
	async fn create(&self, entity: PermissionEntity) -> Result<String, AppError>;
	async fn update(&self, entity: PermissionEntity) -> Result<String, AppError>;
	async fn delete(&self, id: String) -> Result<String, AppError>;
}
