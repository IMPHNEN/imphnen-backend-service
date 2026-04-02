use std::sync::Arc;
use axum::{
    extract::{Extension, Path, Query},
    http::HeaderMap,
    response::IntoResponse,
};
use serde::Deserialize;
use imphnen_libs::ValidatedJson;
use imphnen_utils::{ApiSuccess, extract_email};
use imphnen_utils::AppError;
use crate::sessions::domain::SessionService;
use super::dto::{
    BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListResponseDto,
    UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
};

#[derive(Deserialize)]
pub struct SessionStatusFilter {
    pub status: Option<String>,
}

#[utoipa::path(
    post,
    path = "/v1/mentors/{id}/sessions/create",
    tag = "sessions",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Mentor ID"),
    ),
    request_body = BookSessionRequestDto,
    responses(
        (status = 201, description = "Session booked successfully", body = BookSessionResponseDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Mentor not found"),
    )
)]
pub async fn post_book_session(
    headers: HeaderMap,
    Extension(service): Extension<Arc<dyn SessionService>>,
    Path(mentor_id): Path<String>,
    ValidatedJson(dto): ValidatedJson<BookSessionRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let user_email = extract_email(&headers)
        .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
    let resp = service.book_session(mentor_id, user_email, dto).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    get,
    path = "/v1/mentors/{id}/sessions",
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
    let resp = service.get_mentor_sessions(mentor_id, filter.status).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    get,
    path = "/v1/mentors/{id}/availability",
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
    let resp = service.get_mentor_availability(mentor_id).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    put,
    path = "/v1/sessions/update/{id}/status",
    tag = "sessions",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Session ID"),
    ),
    request_body = UpdateSessionStatusRequestDto,
    responses(
        (status = 200, description = "Status updated successfully", body = UpdateSessionStatusResponseDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn put_update_session_status(
    headers: HeaderMap,
    Extension(service): Extension<Arc<dyn SessionService>>,
    Path(session_id): Path<String>,
    ValidatedJson(dto): ValidatedJson<UpdateSessionStatusRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let user_email = extract_email(&headers)
        .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
    let resp = service.update_session_status(session_id, user_email, dto).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    post,
    path = "/v1/sessions/{id}/feedback/create",
    tag = "sessions",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Session ID"),
    ),
    request_body = SessionFeedbackRequestDto,
    responses(
        (status = 200, description = "Feedback submitted successfully", body = SessionFeedbackResponseDto),
        (status = 400, description = "Invalid request or session not completed"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn post_submit_feedback(
    headers: HeaderMap,
    Extension(service): Extension<Arc<dyn SessionService>>,
    Path(session_id): Path<String>,
    ValidatedJson(dto): ValidatedJson<SessionFeedbackRequestDto>,
) -> Result<impl IntoResponse, AppError> {
    let user_email = extract_email(&headers)
        .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
    let resp = service.submit_feedback(session_id, user_email, dto).await?;
    Ok(ApiSuccess(resp))
}

#[utoipa::path(
    get,
    path = "/v1/users/me/sessions",
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
    let resp = service.get_user_sessions(user_email, filter.status).await?;
    Ok(ApiSuccess(resp))
}
