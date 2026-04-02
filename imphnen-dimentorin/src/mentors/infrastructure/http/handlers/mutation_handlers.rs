use super::super::dto::{
	MentorDetailResponseDto, MentorRegisterResponseDto, MentorUpdateRequestDto,
	MentorUserRegisterRequestDto, MentorVerifyRequestDto,
};
use crate::mentors::domain::MentorService;
use axum::{
	extract::{Extension, Path},
	http::HeaderMap,
	response::{IntoResponse, Response},
};
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::AppError;
use imphnen_utils::{ApiMessage, ApiSuccess, extract_email};
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    post,
    path = "/v1/dimentorin/mentors/create",
    request_body = MentorUserRegisterRequestDto,
    responses(
        (status = 200, description = "[PUBLIC] Mentor registered successfully", body = MentorRegisterResponseDto),
        (status = 400, description = "[PUBLIC] Bad request - validation error"),
        (status = 409, description = "[PUBLIC] Conflict - user already has mentor profile"),
        (status = 500, description = "[PUBLIC] Internal server error")
    ),
    tag = "Mentors"
)]
pub async fn post_register_mentor(
	Extension(service): Extension<Arc<dyn MentorService>>,
	ValidatedJson(dto): ValidatedJson<MentorUserRegisterRequestDto>,
) -> Response {
	match service.register(dto.into()).await {
		Ok(resp) => axum::response::IntoResponse::into_response(
			imphnen_utils::ApiSuccess(MentorRegisterResponseDto::from(resp)),
		),
		Err(e) => ApiMessage::new(e.status_code(), e.to_string()).into_response(),
	}
}

#[utoipa::path(
    put,
    path = "/v1/dimentorin/mentors/update/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Mentor updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[ADMIN] Bad request - validation error"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
    ),
    tag = "Mentors - Admin",
    security(("Bearer" = []))
)]
pub async fn put_update_mentor(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
	Path(id): Path<String>,
	ValidatedJson(dto): ValidatedJson<MentorUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let mentor_uuid = Uuid::parse_str(&id).map_err(|_| {
		AppError::BadRequestError(
			"Invalid mentor ID format. Must be a valid UUID.".to_string(),
		)
	})?;
	require_permissions!(headers, state, [PermissionsEnum::UpdateMentors], {
		let result =
			MentorDetailResponseDto::from(service.update(mentor_uuid, dto.into()).await?);
		Ok(ApiSuccess(result))
	})
}

#[utoipa::path(
    delete,
    path = "/v1/dimentorin/mentors/delete/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Mentor deleted successfully"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
    ),
    tag = "Mentors - Admin",
    security(("Bearer" = []))
)]
pub async fn delete_mentor(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
	Path(id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	let mentor_uuid = Uuid::parse_str(&id).map_err(|_| {
		AppError::BadRequestError(
			"Invalid mentor ID format. Must be a valid UUID.".to_string(),
		)
	})?;
	require_permissions!(headers, state, [PermissionsEnum::DeleteMentors], {
		service.delete(mentor_uuid).await?;
		Ok(ApiMessage::ok("Mentor deleted successfully"))
	})
}

#[utoipa::path(
    put,
    path = "/v1/dimentorin/mentors/verify/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    request_body = MentorVerifyRequestDto,
    responses(
        (status = 200, description = "[ADMIN] Mentor verified successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[ADMIN] Bad request - validation error"),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
    ),
    tag = "Mentors - Admin",
    security(("Bearer" = []))
)]
pub async fn put_verify_mentor(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
	Path(id): Path<String>,
	ValidatedJson(dto): ValidatedJson<MentorVerifyRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	let mentor_uuid = Uuid::parse_str(&id).map_err(|_| {
		AppError::BadRequestError(
			"Invalid mentor ID format. Must be a valid UUID.".to_string(),
		)
	})?;
	require_permissions!(headers, state, [PermissionsEnum::VerifyMentors], {
		let result =
			MentorDetailResponseDto::from(service.verify(mentor_uuid, dto.into()).await?);
		Ok(ApiSuccess(result))
	})
}

#[utoipa::path(
    put,
    path = "/v1/dimentorin/mentors/me/update",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 200, description = "[MENTOR] Mentor profile updated successfully", body = MentorDetailResponseDto),
        (status = 400, description = "[MENTOR] Bad request - validation error"),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 404, description = "[MENTOR] Mentor profile not found"),
        (status = 500, description = "[MENTOR] Internal server error")
    ),
    tag = "Mentors",
    security(("Bearer" = []))
)]
pub async fn put_update_mentor_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
	ValidatedJson(dto): ValidatedJson<MentorUpdateRequestDto>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(
		headers.clone(),
		state,
		[PermissionsEnum::UpdateOwnMentorProfile],
		{
			let email = extract_email(&headers).ok_or_else(|| {
				AppError::AuthenticationError("Token tidak valid".to_string())
			})?;
			let resp =
				MentorDetailResponseDto::from(service.update_me(&email, dto.into()).await?);
			Ok(ApiSuccess(resp))
		}
	)
}

#[utoipa::path(
    put,
    path = "/v1/dimentorin/mentors/update",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 400, description = "[PUBLIC] Bad request - Mentor ID is required for update"),
    ),
    tag = "Mentors - Admin"
)]
pub async fn put_update_mentor_no_id() -> impl IntoResponse {
	ApiMessage::new(
		axum::http::StatusCode::BAD_REQUEST,
		"Mentor ID is required for update",
	)
}
