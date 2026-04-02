use super::super::dto::{MentorAvailabilityDto, SessionListResponseDto};
use crate::sessions::domain::SessionService;
use axum::{
	extract::{Extension, Path, Query},
	http::HeaderMap,
	response::IntoResponse,
};
use imphnen_utils::AppError;
use imphnen_utils::{ApiSuccess, extract_email};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SessionStatusFilter {
	pub status: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/dimentorin/mentors/{id}/sessions",
    tag = "sessions",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Mentor ID"),
        ("status" = Option<String>, Query, description = "Filter by status"),
    ),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = SessionListResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Mentor not found"),
    )
)]
pub async fn get_mentor_sessions(
	headers: HeaderMap,
	Extension(service): Extension<Arc<dyn SessionService>>,
	Path(mentor_id): Path<String>,
	Query(filter): Query<SessionStatusFilter>,
) -> Result<impl IntoResponse, AppError> {
	let _user_email = extract_email(&headers)
		.ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
	let resp = SessionListResponseDto::from(
		service
			.get_mentor_sessions(mentor_id, filter.status)
			.await?,
	);
	Ok(ApiSuccess(resp))
}

#[utoipa::path(
    get,
    path = "/v1/dimentorin/mentors/{id}/availability",
    tag = "sessions",
    params(
        ("id" = String, Path, description = "Mentor ID"),
    ),
    responses(
        (status = 200, description = "Availability retrieved successfully", body = MentorAvailabilityDto),
        (status = 404, description = "Mentor not found"),
    )
)]
pub async fn get_mentor_availability(
	Extension(service): Extension<Arc<dyn SessionService>>,
	Path(mentor_id): Path<String>,
) -> Result<impl IntoResponse, AppError> {
	let resp =
		MentorAvailabilityDto::from(service.get_mentor_availability(mentor_id).await?);
	Ok(ApiSuccess(resp))
}

#[utoipa::path(
    get,
    path = "/v1/dimentorin/sessions/me",
    tag = "sessions",
    security(("Bearer" = [])),
    params(
        ("status" = Option<String>, Query, description = "Filter by status"),
    ),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = SessionListResponseDto),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn get_my_sessions(
	headers: HeaderMap,
	Extension(service): Extension<Arc<dyn SessionService>>,
	Query(filter): Query<SessionStatusFilter>,
) -> Result<impl IntoResponse, AppError> {
	let user_email = extract_email(&headers)
		.ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
	let resp = SessionListResponseDto::from(
		service.get_user_sessions(user_email, filter.status).await?,
	);
	Ok(ApiSuccess(resp))
}
