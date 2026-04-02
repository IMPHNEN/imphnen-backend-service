use super::super::dto::{MentorDetailResponseDto, MentorListResponseDto};
use crate::mentors::domain::MentorService;
use axum::{
	extract::{Extension, Path},
	http::HeaderMap,
	response::IntoResponse,
};
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_libs::AppState;
use imphnen_utils::AppError;
use imphnen_utils::{ApiMessage, ApiPaginated, ApiSuccess, extract_email};
use paginator_axum::PaginationQuery;
use paginator_utils::PaginatorResponse;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/v1/mentors",
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("per_page" = Option<u64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search query"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Sort order (ASC/DESC)"),
    ),
    responses(
        (status = 200, description = "[ADMIN] Get list of mentors", body = Vec<MentorListResponseDto>),
        (status = 500, description = "[ADMIN] Internal server error")
    ),
    tag = "Mentors",
    security(("Bearer" = []))
)]
pub async fn get_mentor_list(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
	PaginationQuery(params): PaginationQuery,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(headers, state, [PermissionsEnum::ReadListMentors], {
		let result = service.list(params).await?;
		let mapped = PaginatorResponse {
			data: result
				.data
				.into_iter()
				.map(MentorListResponseDto::from)
				.collect(),
			meta: result.meta,
		};
		Ok(ApiPaginated(mapped))
	})
}

#[utoipa::path(
    get,
    path = "/v1/mentors/detail/{id}",
    params(
        ("id" = String, Path, description = "Mentor ID")
    ),
    responses(
        (status = 200, description = "[ADMIN] Get mentor by ID", body = MentorDetailResponseDto),
        (status = 404, description = "[ADMIN] Mentor not found"),
        (status = 500, description = "[ADMIN] Internal server error")
    ),
    tag = "Mentors",
    security(("Bearer" = []))
)]
pub async fn get_mentor_by_id(
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
	require_permissions!(headers, state, [PermissionsEnum::ReadDetailMentors], {
		let dto = MentorDetailResponseDto::from(service.get_by_id(mentor_uuid).await?);
		Ok(ApiSuccess(dto))
	})
}

#[utoipa::path(
    get,
    path = "/v1/mentors/me",
    responses(
        (status = 200, description = "[MENTOR] Current user's mentor profile", body = MentorDetailResponseDto),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 403, description = "[MENTOR] Mentor profile not found for current user"),
        (status = 500, description = "[MENTOR] Internal server error")
    ),
    tag = "Mentors",
    security(("Bearer" = []))
)]
pub async fn get_mentor_me(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(
		headers.clone(),
		state,
		[PermissionsEnum::ReadOwnMentorProfile],
		{
			let email = extract_email(&headers).ok_or_else(|| {
				AppError::AuthenticationError("Token tidak valid".to_string())
			})?;
			let detail = service.get_by_email(&email).await.map_err(|_| {
				AppError::ForbiddenError(
					"Mentor profile not found for current user".to_string(),
				)
			})?;
			Ok(ApiSuccess(MentorDetailResponseDto::from(detail)))
		}
	)
}

#[utoipa::path(
    get,
    path = "/v1/mentors/me/status",
    responses(
        (status = 200, description = "[MENTOR] Mentor application status", body = String),
        (status = 401, description = "[MENTOR] Unauthorized - invalid token"),
        (status = 403, description = "[MENTOR] No mentor application found for current user"),
        (status = 500, description = "[MENTOR] Internal server error")
    ),
    tag = "Mentors",
    security(("Bearer" = []))
)]
pub async fn get_mentor_status(
	headers: HeaderMap,
	Extension(state): Extension<AppState>,
	Extension(service): Extension<Arc<dyn MentorService>>,
) -> Result<impl IntoResponse, AppError> {
	require_permissions!(
		headers.clone(),
		state,
		[PermissionsEnum::ReadOwnMentorStatus],
		{
			let email = extract_email(&headers).ok_or_else(|| {
				AppError::AuthenticationError("Token tidak valid".to_string())
			})?;
			let status = service.get_status(&email).await.map_err(|_| {
				AppError::ForbiddenError(
					"No mentor application found for current user".to_string(),
				)
			})?;
			Ok(ApiMessage::ok(&status))
		}
	)
}
