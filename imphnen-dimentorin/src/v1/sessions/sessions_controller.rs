use super::{
    BookSessionRequestDto, BookSessionResponseDto, MentorAvailabilityDto,
    SessionFeedbackRequestDto, SessionFeedbackResponseDto, SessionListResponseDto,
    SessionsService, UpdateSessionStatusRequestDto, UpdateSessionStatusResponseDto,
};
use axum::{
    extract::{Extension, Path, Query},
    http::HeaderMap,
    response::Response,
    routing::{get, post, put},
    Json, Router,
};
use imphnen_libs::AppState;
use imphnen_utils::extract_email;
use serde::Deserialize;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        post_book_session,
        get_mentor_sessions,
        get_mentor_availability,
        put_update_session_status,
        post_submit_feedback,
        get_my_sessions,
    ),
    components(schemas(
        BookSessionRequestDto,
        BookSessionResponseDto,
        SessionListResponseDto,
        super::SessionListItemDto,
        MentorAvailabilityDto,
        super::AvailabilitySlotDto,
        UpdateSessionStatusRequestDto,
        UpdateSessionStatusResponseDto,
        SessionFeedbackRequestDto,
        SessionFeedbackResponseDto,
    )),
    tags(
        (name = "sessions", description = "Mentoring Sessions Management API")
    )
)]
pub struct SessionsApiDoc;

// ============================================
// Book Session
// ============================================

#[utoipa::path(
    post,
    path = "/v1/mentors/{id}/sessions/book",
    tag = "sessions",
    summary = "Book a mentoring session",
    description = "Book a mentoring session with a specific mentor. Requires authentication.",
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
    Extension(state): Extension<AppState>,
    Path(mentor_id): Path<String>,
    Json(dto): Json<BookSessionRequestDto>,
) -> Response {
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => {
            return imphnen_utils::common_response(
                axum::http::StatusCode::UNAUTHORIZED,
                "Token tidak valid",
            );
        }
    };
    
    SessionsService::book_session(&state, mentor_id, user_email, dto).await
}

// ============================================
// Get Mentor's Sessions
// ============================================

#[derive(Deserialize)]
pub struct SessionStatusFilter {
    status: Option<String>,
}

#[utoipa::path(
    get,
    path = "/v1/mentors/{id}/sessions",
    tag = "sessions",
    summary = "List mentor's sessions",
    description = "Get all sessions for a specific mentor. Only accessible by the mentor themselves or admin.",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Mentor ID"),
        ("status" = Option<String>, Query, description = "Filter by status (pending, confirmed, completed, cancelled, no_show)"),
    ),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = SessionListResponseDto),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Mentor not found"),
    )
)]
pub async fn get_mentor_sessions(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(mentor_id): Path<String>,
    Query(filter): Query<SessionStatusFilter>,
) -> Response {
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => {
            return imphnen_utils::common_response(
                axum::http::StatusCode::UNAUTHORIZED,
                "Token tidak valid",
            );
        }
    };
    
    SessionsService::get_mentor_sessions(&state, mentor_id, user_email, filter.status).await
}

// ============================================
// Get Mentor Availability
// ============================================

#[utoipa::path(
    get,
    path = "/v1/mentors/{id}/availability",
    tag = "sessions",
    summary = "Get mentor availability",
    description = "Get available time slots for booking with a mentor. Public endpoint.",
    params(
        ("id" = String, Path, description = "Mentor ID"),
    ),
    responses(
        (status = 200, description = "Availability retrieved successfully", body = MentorAvailabilityDto),
        (status = 404, description = "Mentor not found"),
    )
)]
pub async fn get_mentor_availability(
    Extension(state): Extension<AppState>,
    Path(mentor_id): Path<String>,
) -> Response {
    SessionsService::get_mentor_availability(&state, mentor_id).await
}

// ============================================
// Update Session Status
// ============================================

#[utoipa::path(
    put,
    path = "/v1/sessions/{id}/status",
    tag = "sessions",
    summary = "Update session status",
    description = "Update the status of a session (confirm, complete, cancel). Only accessible by the mentor.",
    security(("Bearer" = [])),
    params(
        ("id" = String, Path, description = "Session ID"),
    ),
    request_body = UpdateSessionStatusRequestDto,
    responses(
        (status = 200, description = "Status updated successfully", body = UpdateSessionStatusResponseDto),
        (status = 400, description = "Invalid request"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
        (status = 404, description = "Session not found"),
    )
)]
pub async fn put_update_session_status(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(session_id): Path<String>,
    Json(dto): Json<UpdateSessionStatusRequestDto>,
) -> Response {
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => {
            return imphnen_utils::common_response(
                axum::http::StatusCode::UNAUTHORIZED,
                "Token tidak valid",
            );
        }
    };
    
    SessionsService::update_session_status(&state, session_id, user_email, dto).await
}

// ============================================
// Submit Feedback
// ============================================

#[utoipa::path(
    post,
    path = "/v1/sessions/{id}/feedback",
    tag = "sessions",
    summary = "Submit session feedback",
    description = "Submit feedback and rating for a completed session. Only accessible by the mentee.",
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
    Extension(state): Extension<AppState>,
    Path(session_id): Path<String>,
    Json(dto): Json<SessionFeedbackRequestDto>,
) -> Response {
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => {
            return imphnen_utils::common_response(
                axum::http::StatusCode::UNAUTHORIZED,
                "Token tidak valid",
            );
        }
    };
    
    SessionsService::submit_feedback(&state, session_id, user_email, dto).await
}

// ============================================
// Get User's Sessions
// ============================================

#[utoipa::path(
    get,
    path = "/v1/users/me/sessions",
    tag = "sessions",
    summary = "Get my sessions",
    description = "Get all sessions for the authenticated user (as mentee). Requires authentication.",
    security(("Bearer" = [])),
    params(
        ("status" = Option<String>, Query, description = "Filter by status (pending, confirmed, completed, cancelled, no_show)"),
    ),
    responses(
        (status = 200, description = "Sessions retrieved successfully", body = SessionListResponseDto),
        (status = 401, description = "Unauthorized"),
    )
)]
pub async fn get_my_sessions(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Query(filter): Query<SessionStatusFilter>,
) -> Response {
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => {
            return imphnen_utils::common_response(
                axum::http::StatusCode::UNAUTHORIZED,
                "Token tidak valid",
            );
        }
    };
    
    SessionsService::get_user_sessions(&state, user_email, filter.status).await
}

// ============================================
// Router
// ============================================

pub fn sessions_router() -> Router {
    Router::new()
        // Book session (under mentors path)
        .route("/mentors/:id/sessions/book", post(post_book_session))
        // Get mentor's sessions
        .route("/mentors/:id/sessions", get(get_mentor_sessions))
        // Get mentor availability (public - no auth)
        .route("/mentors/:id/availability", get(get_mentor_availability))
        // Update session status
        .route("/sessions/:id/status", put(put_update_session_status))
        // Submit feedback
        .route("/sessions/:id/feedback", post(post_submit_feedback))
        // Get my sessions
        .route("/users/me/sessions", get(get_my_sessions))
}
