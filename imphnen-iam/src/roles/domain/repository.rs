use super::role::RoleEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;

#[async_trait]
pub trait RoleRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<RoleEntity>, AppError>;
	async fn find_by_id(&self, id: String) -> Result<RoleEntity, AppError>;
	async fn find_by_name(&self, name: String) -> Result<RoleEntity, AppError>;
	async fn create(&self, entity: RoleEntity) -> Result<RoleEntity, AppError>;
	async fn update(
		&self,
		id: String,
		name: Option<String>,
		permissions: Option<Vec<String>>,
	) -> Result<String, AppError>;
	async fn delete(&self, id: String) -> Result<String, AppError>;
}
