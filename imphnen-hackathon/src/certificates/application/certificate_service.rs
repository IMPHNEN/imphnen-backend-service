use crate::certificates::domain::entity::CertificateData;
use crate::certificates::domain::repository::CertificateRepository;
use crate::certificates::domain::service::CertificateService;
use async_trait::async_trait;
use imphnen_utils::errors::AppError;
use std::sync::Arc;
use uuid::Uuid;

pub struct CertificateServiceImpl {
	repo: Arc<dyn CertificateRepository>,
}

impl CertificateServiceImpl {
	pub fn new(repo: Arc<dyn CertificateRepository>) -> Self {
		Self { repo }
	}
}

#[async_trait]
impl CertificateService for CertificateServiceImpl {
	async fn get_certificate(
		&self,
		user_id: Uuid,
	) -> Result<CertificateData, AppError> {
		self
			.repo
			.find_by_user_id(user_id)
			.await?
			.ok_or_else(|| AppError::NotFoundError("User not found".to_string()))
	}
}
