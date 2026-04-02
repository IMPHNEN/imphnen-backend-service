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
        (status = 200, description = "Get certificate for user",
         body = inline(CertificateResponse),
         example = json!({
             "data": {
                 "user_id": "3fa85f64-5717-4562-b3fc-2c963f66afa6",
                 "fullname": "Budi Santoso",
                 "email": "budi@example.com",
                 "avatar": "https://cdn.example.com/avatar.png",
                 "team_id": "7c3a1d2e-8f4b-4c5a-9d6e-1f2a3b4c5d6e",
                 "team_name": "Rust Enjoyers",
                 "is_leader": true,
                 "project_name": "EcoTrack - Sustainability Monitor",
                 "submission_status": "confirmed",
                 "winner_rank": 1,
                 "winner_prize": "Rp 10.000.000"
             },
             "version": "0.3.0"
         })),
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
