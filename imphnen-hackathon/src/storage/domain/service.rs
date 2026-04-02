use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait StorageService: Send + Sync {
	async fn upload(
		&self,
		folder: &str,
		user_id: Uuid,
		filename: &str,
		content_type: &str,
		data_base64: &str,
	) -> Result<String, AppError>;
}
