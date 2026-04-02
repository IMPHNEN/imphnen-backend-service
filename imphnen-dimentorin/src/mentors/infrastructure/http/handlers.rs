use std::sync::Arc;
use axum::{
    extract::{Extension, Path},
    http::HeaderMap,
    response::{IntoResponse, Response},
};
use paginator_axum::PaginationQuery;
use uuid::Uuid;
use imphnen_libs::{AppState, ValidatedJson};
use imphnen_utils::{ApiSuccess, ApiPaginated, ApiMessage, extract_email};
use imphnen_iam::{PermissionsEnum, require_permissions};
use imphnen_utils::AppError;
use crate::mentors::domain::MentorService;
use super::dto::{
    MentorDetailResponseDto, MentorListResponseDto, MentorRegisterResponseDto,
    MentorUpdateRequestDto, MentorUserRegisterRequestDto, MentorVerifyRequestDto,
};

#[utoipa::path(
    post,
    path = "/v1/mentors/create",
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
    match service.register(dto).await {
        Ok(resp) => ApiSuccess(resp).into_response(),
        Err(e) => ApiMessage::new(e.status_code(), e.to_string()).into_response(),
    }
}

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
        Ok(ApiPaginated(result))
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
    let mentor_uuid = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequestError("Invalid mentor ID format. Must be a valid UUID.".to_string()))?;
    require_permissions!(headers, state, [PermissionsEnum::ReadDetailMentors], {
        let dto = service.get_by_id(mentor_uuid).await?;
        Ok(ApiSuccess(dto))
    })
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update/{id}",
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
    let mentor_uuid = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequestError("Invalid mentor ID format. Must be a valid UUID.".to_string()))?;
    require_permissions!(headers, state, [PermissionsEnum::UpdateMentors], {
        let result = service.update(mentor_uuid, dto).await?;
        Ok(ApiSuccess(result))
    })
}

#[utoipa::path(
    delete,
    path = "/v1/mentors/delete/{id}",
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
    let mentor_uuid = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequestError("Invalid mentor ID format. Must be a valid UUID.".to_string()))?;
    require_permissions!(headers, state, [PermissionsEnum::DeleteMentors], {
        service.delete(mentor_uuid).await?;
        Ok(ApiMessage::ok("Mentor deleted successfully"))
    })
}

#[utoipa::path(
    put,
    path = "/v1/mentors/verify/{id}",
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
    let mentor_uuid = Uuid::parse_str(&id)
        .map_err(|_| AppError::BadRequestError("Invalid mentor ID format. Must be a valid UUID.".to_string()))?;
    require_permissions!(headers, state, [PermissionsEnum::VerifyMentors], {
        let result = service.verify(mentor_uuid, dto).await?;
        Ok(ApiSuccess(result))
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
    require_permissions!(headers.clone(), state, [PermissionsEnum::ReadOwnMentorProfile], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
        let dto = service.get_by_email(&email).await
            .map_err(|_| AppError::ForbiddenError("Mentor profile not found for current user".to_string()))?;
        Ok(ApiSuccess(dto))
    })
}

#[utoipa::path(
    put,
    path = "/v1/mentors/me/update",
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
    require_permissions!(headers.clone(), state, [PermissionsEnum::UpdateOwnMentorProfile], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
        let resp = service.update_me(&email, dto).await?;
        Ok(ApiSuccess(resp))
    })
}

#[utoipa::path(
    put,
    path = "/v1/mentors/update",
    request_body = MentorUpdateRequestDto,
    responses(
        (status = 400, description = "[PUBLIC] Bad request - Mentor ID is required for update"),
    ),
    tag = "Mentors - Admin"
)]
pub async fn put_update_mentor_no_id() -> impl IntoResponse {
    ApiMessage::new(axum::http::StatusCode::BAD_REQUEST, "Mentor ID is required for update")
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
    require_permissions!(headers.clone(), state, [PermissionsEnum::ReadOwnMentorStatus], {
        let email = extract_email(&headers)
            .ok_or_else(|| AppError::AuthenticationError("Token tidak valid".to_string()))?;
        let status = service.get_status(&email).await
            .map_err(|_| AppError::ForbiddenError("No mentor application found for current user".to_string()))?;
        Ok(ApiMessage::ok(&status))
    })
}
