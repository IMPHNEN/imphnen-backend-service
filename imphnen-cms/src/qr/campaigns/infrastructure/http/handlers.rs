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
