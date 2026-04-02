use super::entity::CertificateData;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use uuid::Uuid;

#[async_trait]
pub trait CertificateService: Send + Sync {
	async fn get_certificate(
		&self,
		user_id: Uuid,
	) -> Result<CertificateData, AppError>;
}
