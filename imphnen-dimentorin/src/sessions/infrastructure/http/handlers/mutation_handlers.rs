use super::super::dto::{
	BookSessionRequestDto, BookSessionResponseDto, SessionFeedbackRequestDto,
	SessionFeedbackResponseDto, UpdateSessionStatusRequestDto,
	UpdateSessionStatusResponseDto,
};
use crate::sessions::domain::SessionService;
use axum::{
	extract::{Extension, Path},
	http::{HeaderMap, header::AUTHORIZATION},
	response::IntoResponse,
};
use imphnen_libs::{ValidatedJson, decode_access_token};
use imphnen_utils::AppError;
use imphnen_utils::ApiSuccess;
use std::sync::Arc;

fn extract_user_id(headers: &HeaderMap) -> Result<String, AppError> {
	let token = headers
		.get(AUTHORIZATION)
		.and_then(|h| h.to_str().ok())
		.and_then(|s| s.strip_prefix("Bearer "))
		.ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
	let claims = decode_access_token(token)
		.map_err(|_| AppError::AuthenticationError("Token tidak valid".to_string()))?;
	Ok(claims.claims.user_id)
}

#[utoipa::path(
    post,
    path = "/v1/dimentorin/mentors/{id}/sessions/create",
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
	let user_id = extract_user_id(&headers)?;
	let resp = BookSessionResponseDto::from(
		service
			.book_session(mentor_id, user_id, dto.into())
			.await?,
	);
	Ok(ApiSuccess(resp))
}

#[utoipa::path(
    put,
    path = "/v1/dimentorin/sessions/update/{id}/status",
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
	let user_id = extract_user_id(&headers)?;
	let resp = UpdateSessionStatusResponseDto::from(
		service
			.update_session_status(session_id, user_id, dto.into())
			.await?,
	);
	Ok(ApiSuccess(resp))
}

#[utoipa::path(
    post,
    path = "/v1/dimentorin/sessions/{id}/feedback/create",
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
	let user_id = extract_user_id(&headers)?;
	let resp = SessionFeedbackResponseDto::from(
		service
			.submit_feedback(session_id, user_id, dto.into())
			.await?,
	);
	Ok(ApiSuccess(resp))
}
