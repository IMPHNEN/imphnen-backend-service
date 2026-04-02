use super::entity::CertificateData;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait CertificateRepository: Send + Sync {
	async fn find_by_user_id(
		&self,
		user_id: Uuid,
	) -> Result<Option<CertificateData>, AppError>;
}
