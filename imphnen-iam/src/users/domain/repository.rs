use super::user::UserEntity;
use async_trait::async_trait;
use imphnen_utils::AppError;
use paginator_rs::PaginationParams;
use paginator_utils::PaginatorResponse;

#[derive(Clone, Debug)]
pub struct UserListItem {
	pub id: String,
	pub role: String,
	pub fullname: String,
	pub email: String,
	pub avatar: Option<String>,
	pub is_active: bool,
	pub created_at: String,
	pub updated_at: String,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
	async fn find_all(
		&self,
		params: PaginationParams,
	) -> Result<PaginatorResponse<UserListItem>, AppError>;
	async fn find_by_id(&self, id: &str) -> Result<UserEntity, AppError>;
	async fn find_by_email(&self, email: String) -> Result<UserEntity, AppError>;
	async fn create(&self, entity: UserEntity) -> Result<String, AppError>;
	async fn update(&self, entity: UserEntity) -> Result<String, AppError>;
	async fn delete(&self, id: String) -> Result<String, AppError>;
}
