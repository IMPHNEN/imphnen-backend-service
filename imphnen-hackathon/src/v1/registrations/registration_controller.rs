use axum::{
    extract::{Extension, Path},
    http::{HeaderMap, StatusCode},
    response::Response,
    routing::{get, post, put},
    Json, Router,
};
use imphnen_entities::ResponseSuccessDto;
use imphnen_libs::AppState;
use imphnen_utils::{common_response, extract_email, make_thing_from_enum};
use imphnen_libs::ResourceEnum;

use super::{
    CheckInResponseDto, RegistrationListResponseDto, RegistrationRequestDto,
    RegistrationResponseDto, RegistrationStatsDto, RegistrationsService,
    UpdateRegistrationStatusRequestDto, UpdateRegistrationStatusResponseDto,
    UserHackathonsResponseDto,
};

// ============================================
// POST /v1/hackathons/{id}/register
// ============================================
#[utoipa::path(
    post,
    path = "/v1/hackathons/{id}/register",
    tag = "registrations",
    summary = "Register for a hackathon",
    description = "Submit a registration for a hackathon. User must be authenticated.",
    params(
        ("id" = String, Path, description = "Hackathon ID")
    ),
    request_body = RegistrationRequestDto,
    responses(
        (status = 200, description = "Registration submitted successfully", body = ResponseSuccessDto<RegistrationResponseDto>),
        (status = 400, description = "Invalid input or validation error"),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 409, description = "User already registered for this hackathon"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn post_register_hackathon(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    Json(data): Json<RegistrationRequestDto>,
) -> Response {
    // Authentication
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    // Parse hackathon ID
    let hackathon_id = make_thing_from_enum(ResourceEnum::Hackathons, &id);

    let service = RegistrationsService::new(&state);
    service.register_hackathon(&hackathon_id, &user_email, data).await
}

// ============================================
// GET /v1/hackathons/{id}/registrations
// ============================================
#[utoipa::path(
    get,
    path = "/v1/hackathons/{id}/registrations",
    tag = "registrations",
    summary = "List hackathon registrations",
    description = "Get all registrations for a hackathon. Requires admin/organizer permissions. Optional status filter.",
    params(
        ("id" = String, Path, description = "Hackathon ID"),
        ("status" = Option<String>, Query, description = "Filter by status: pending, approved, rejected, waitlisted, cancelled")
    ),
    responses(
        (status = 200, description = "Registrations retrieved successfully", body = ResponseSuccessDto<RegistrationListResponseDto>),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_hackathon_registrations(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
    // Authentication
    match extract_email(&headers) {
        Some(_) => {},
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    // Parse hackathon ID
    let hackathon_id = make_thing_from_enum(ResourceEnum::Hackathons, &id);
    
    let status_filter = params.get("status").cloned();

    let service = RegistrationsService::new(&state);
    service.get_hackathon_registrations(&hackathon_id, status_filter).await
}

// ============================================
// GET /v1/users/me/hackathons
// ============================================
#[utoipa::path(
    get,
    path = "/v1/users/me/hackathons",
    tag = "registrations",
    summary = "Get my hackathon registrations",
    description = "Get all hackathons the current user has registered for.",
    responses(
        (status = 200, description = "Hackathons retrieved successfully", body = ResponseSuccessDto<UserHackathonsResponseDto>),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_my_hackathons(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
) -> Response {
    // Authentication
    let user_email = match extract_email(&headers) {
        Some(email) => email,
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    let service = RegistrationsService::new(&state);
    service.get_my_hackathons(&user_email).await
}

// ============================================
// PUT /v1/hackathons/{hackathon_id}/registrations/{registration_id}/status
// ============================================
#[utoipa::path(
    put,
    path = "/v1/hackathons/{hackathon_id}/registrations/{registration_id}/status",
    tag = "registrations",
    summary = "Update registration status",
    description = "Approve, reject, or update the status of a registration. Requires admin/organizer permissions.",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("registration_id" = String, Path, description = "Registration ID")
    ),
    request_body = UpdateRegistrationStatusRequestDto,
    responses(
        (status = 200, description = "Status updated successfully", body = ResponseSuccessDto<UpdateRegistrationStatusResponseDto>),
        (status = 400, description = "Invalid input or validation error"),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 404, description = "Registration not found"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn put_update_registration_status(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path((_hackathon_id, registration_id)): Path<(String, String)>,
    Json(data): Json<UpdateRegistrationStatusRequestDto>,
) -> Response {
    // Authentication
    match extract_email(&headers) {
        Some(_) => {},
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    // Parse registration ID
    let reg_id = make_thing_from_enum(ResourceEnum::HackathonRegistrations, &registration_id);
    
    let service = RegistrationsService::new(&state);
    service.update_registration_status(&reg_id, data).await
}

// ============================================
// POST /v1/hackathons/{hackathon_id}/registrations/{registration_id}/check-in
// ============================================
#[utoipa::path(
    post,
    path = "/v1/hackathons/{hackathon_id}/registrations/{registration_id}/check-in",
    tag = "registrations",
    summary = "Check-in participant",
    description = "Mark a participant as checked in for the hackathon. Requires admin/organizer permissions.",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("registration_id" = String, Path, description = "Registration ID")
    ),
    responses(
        (status = 200, description = "Participant checked in successfully", body = ResponseSuccessDto<CheckInResponseDto>),
        (status = 400, description = "Invalid request - participant not approved or already checked in"),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 404, description = "Registration not found"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn post_check_in_participant(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path((_hackathon_id, registration_id)): Path<(String, String)>,
) -> Response {
    // Authentication
    match extract_email(&headers) {
        Some(_) => {},
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    // Parse registration ID
    let reg_id = make_thing_from_enum(ResourceEnum::HackathonRegistrations, &registration_id);
    
    let service = RegistrationsService::new(&state);
    service.check_in_participant(&reg_id).await
}

// ============================================
// GET /v1/hackathons/{id}/registrations/stats
// ============================================
#[utoipa::path(
    get,
    path = "/v1/hackathons/{id}/registrations/stats",
    tag = "registrations",
    summary = "Get registration statistics",
    description = "Get comprehensive statistics about hackathon registrations. Requires admin/organizer permissions.",
    params(
        ("id" = String, Path, description = "Hackathon ID")
    ),
    responses(
        (status = 200, description = "Statistics retrieved successfully", body = ResponseSuccessDto<RegistrationStatsDto>),
        (status = 400, description = "Invalid input"),
        (status = 401, description = "Unauthorized - authentication required"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_registration_stats(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    Path(id): Path<String>,
) -> Response {
    // Authentication
    match extract_email(&headers) {
        Some(_) => {},
        None => return common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    };
    
    // Parse hackathon ID
    let hackathon_id = make_thing_from_enum(ResourceEnum::Hackathons, &id);
    
    let service = RegistrationsService::new(&state);
    service.get_registration_stats(&hackathon_id).await
}

// ============================================
// Router
// ============================================
pub fn registrations_router() -> Router {
    Router::new()
        .route(
            "/hackathons/{id}/register",
            post(post_register_hackathon),
        )
        .route(
            "/hackathons/{id}/registrations",
            get(get_hackathon_registrations),
        )
        .route(
            "/hackathons/{id}/registrations/stats",
            get(get_registration_stats),
        )
        .route(
            "/hackathons/{hackathon_id}/registrations/{registration_id}/status",
            put(put_update_registration_status),
        )
        .route(
            "/hackathons/{hackathon_id}/registrations/{registration_id}/check-in",
            post(post_check_in_participant),
        )
        .route("/users/me/hackathons", get(get_my_hackathons))
}
