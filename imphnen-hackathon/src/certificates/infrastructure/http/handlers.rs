use super::dto::CertificateResponse;
use crate::certificates::domain::service::CertificateService;
use axum::{Extension, extract::Path, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

pub async fn get_certificate_handler(
	Extension(service): Extension<Arc<dyn CertificateService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let cert = service.get_certificate(user_id).await?;
	Ok(ApiSuccess(CertificateResponse::from(cert)).into_response())
}
