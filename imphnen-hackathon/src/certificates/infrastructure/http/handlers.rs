use super::dto::CertificateResponse;
use crate::certificates::domain::service::CertificateService;
use axum::{Extension, extract::Path, response::IntoResponse};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/hackathon/certificates/{user_id}",
    params(("user_id" = Uuid, Path, description = "User ID")),
    responses(
        (status = 200, description = "Get certificate for user"),
        (status = 404, description = "Certificate not found")
    ),
    tag = "Hackathon - Certificates"
)]
pub async fn get_certificate_handler(
	Extension(service): Extension<Arc<dyn CertificateService>>,
	Path(user_id): Path<Uuid>,
) -> Result<axum::response::Response, AppError> {
	let cert = service.get_certificate(user_id).await?;
	Ok(ApiSuccess(CertificateResponse::from(cert)).into_response())
}
