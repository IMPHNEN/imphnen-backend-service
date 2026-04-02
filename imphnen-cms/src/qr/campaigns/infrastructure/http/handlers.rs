use axum::{
	Extension, Json,
	extract::{Multipart, Path},
	response::{IntoResponse, Response},
};
use imphnen_utils::{errors::AppError, response_format::ApiSuccess};
use std::sync::Arc;
use uuid::Uuid;

use crate::qr::{
	campaigns::{
		domain::service::QrCampaignService,
		infrastructure::http::dto::CreateCampaignRequest,
	},
	middleware::qr_auth::QrAuthUser,
};

#[utoipa::path(
    post,
    path = "/v1/qr/campaigns",
    request_body = CreateCampaignRequest,
    responses(
        (status = 201, description = "Create a QR campaign"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Campaigns",
    security(("bearer_auth" = []))
)]
pub async fn create_campaign_handler(
	Extension(service): Extension<Arc<dyn QrCampaignService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Json(body): Json<CreateCampaignRequest>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	let campaign = service
		.create(body.name, body.url, auth_user.user_id)
		.await?;
	Ok(imphnen_utils::response_format::ApiCreated(campaign).into_response())
}

#[utoipa::path(
    get,
    path = "/v1/qr/campaigns",
    responses(
        (status = 200, description = "Admin: list all QR campaigns"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Campaigns",
    security(("bearer_auth" = []))
)]
pub async fn list_campaigns_handler(
	Extension(service): Extension<Arc<dyn QrCampaignService>>,
	Extension(auth_user): Extension<QrAuthUser>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	let campaigns = service.list_all().await?;
	Ok(ApiSuccess(campaigns).into_response())
}

#[utoipa::path(
    put,
    path = "/v1/qr/campaigns/{id}/activate",
    params(("id" = Uuid, Path, description = "Campaign ID")),
    responses(
        (status = 200, description = "Admin: activate a campaign"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Campaigns",
    security(("bearer_auth" = []))
)]
pub async fn activate_campaign_handler(
	Extension(service): Extension<Arc<dyn QrCampaignService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	let campaign = service.set_active(id).await?;
	Ok(ApiSuccess(campaign).into_response())
}

#[utoipa::path(
    delete,
    path = "/v1/qr/campaigns/{id}",
    params(("id" = Uuid, Path, description = "Campaign ID")),
    responses(
        (status = 200, description = "Admin: delete a campaign"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden - admin only")
    ),
    tag = "QR - Campaigns",
    security(("bearer_auth" = []))
)]
pub async fn delete_campaign_handler(
	Extension(service): Extension<Arc<dyn QrCampaignService>>,
	Extension(auth_user): Extension<QrAuthUser>,
	Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
	if auth_user.role != "admin" {
		return Err(AppError::ForbiddenError(
			"Admin access required".to_string(),
		));
	}
	service.delete(id).await?;
	Ok(
		imphnen_utils::response_format::ApiMessage::ok("Campaign deleted successfully")
			.into_response(),
	)
}

#[utoipa::path(
    post,
    path = "/v1/qr/campaigns/process-image",
    responses(
        (status = 200, description = "Process QR code image (multipart/form-data with 'file' field)"),
        (status = 401, description = "Unauthorized")
    ),
    tag = "QR - Campaigns",
    security(("bearer_auth" = []))
)]
pub async fn process_image_handler(
	Extension(service): Extension<Arc<dyn QrCampaignService>>,
	Extension(_auth_user): Extension<QrAuthUser>,
	mut multipart: Multipart,
) -> Result<Response, AppError> {
	let mut image_bytes = Vec::new();
	while let Some(field) = multipart
		.next_field()
		.await
		.map_err(|e| AppError::BadRequestError(e.to_string()))?
	{
		if field.name() == Some("file") {
			image_bytes = field
				.bytes()
				.await
				.map_err(|e| AppError::BadRequestError(e.to_string()))?
				.to_vec();
			break;
		}
	}
	if image_bytes.is_empty() {
		return Err(AppError::BadRequestError("No file provided".to_string()));
	}
	let png_bytes = service.process_image(image_bytes).await?;
	Ok(([(axum::http::header::CONTENT_TYPE, "image/png")], png_bytes).into_response())
}
