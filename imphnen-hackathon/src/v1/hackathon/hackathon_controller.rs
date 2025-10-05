use super::hackathon_dto::{
    HackathonCreateRequestDto, HackathonDto, HackathonEventCreateRequestDto, HackathonEventDto,
    HackathonEventUpdateRequestDto, HackathonSubmissionCreateRequestDto,
    HackathonSubmissionDto, HackathonSubmissionUpdateRequestDto, HackathonTimelineCreateRequestDto,
    HackathonTimelineDto, HackathonTimelineUpdateRequestDto, HackathonUpdateRequestDto,
};
use super::hackathon_service::{HackathonService, HackathonServiceTrait};
use crate::{AppState, ResponseSuccessDto, ErrorDto};
use imphnen_libs::{MetaRequestDto, ResponseListSuccessDto};
use axum::{
    extract::{Extension, Path, Query},
    http::StatusCode,
    Json, Router,
    response::IntoResponse,
    routing::{delete, get, post, put},
};

// Hackathon routes
#[utoipa::path(
    post,
    path = "/v1/hackathons",
    request_body = HackathonCreateRequestDto,
    responses(
        (status = 201, description = "Hackathon created successfully", body = ResponseSuccessDto<HackathonDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathons"
)]
pub async fn create_hackathon(
    Extension(state): Extension<AppState>,
    Json(payload): Json<HackathonCreateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::create_hackathon(payload, &state).await {
        Ok(response) => (axum::http::StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons/{id}",
    params(
        ("id" = String, Path, description = "Hackathon ID")
    ),
    responses(
        (status = 200, description = "Hackathon retrieved successfully", body = ResponseSuccessDto<HackathonDto>),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathons"
)]
pub async fn get_hackathon(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::get_hackathon(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons",
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "Hackathons retrieved successfully", body = ResponseListSuccessDto<Vec<HackathonDto>>),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathons"
)]
pub async fn list_hackathons(
    Extension(state): Extension<AppState>,
    Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match HackathonService::list_hackathons(meta, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/v1/hackathons/{id}",
    params(
        ("id" = String, Path, description = "Hackathon ID")
    ),
    request_body = HackathonUpdateRequestDto,
    responses(
        (status = 200, description = "Hackathon updated successfully", body = ResponseSuccessDto<HackathonDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathons"
)]
pub async fn update_hackathon(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<HackathonUpdateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::update_hackathon(id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/hackathons/{id}",
    params(
        ("id" = String, Path, description = "Hackathon ID")
    ),
    responses(
        (status = 200, description = "Hackathon deleted successfully", body = ResponseSuccessDto<String>),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathons"
)]
pub async fn delete_hackathon(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::delete_hackathon(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

// Hackathon Events routes
#[utoipa::path(
    post,
    path = "/v1/hackathons/{hackathon_id}/events",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID")
    ),
    request_body = HackathonEventCreateRequestDto,
    responses(
        (status = 201, description = "Event created successfully", body = ResponseSuccessDto<HackathonEventDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Events"
)]
pub async fn create_hackathon_event(
    Extension(state): Extension<AppState>,
    Path(hackathon_id): Path<String>,
    Json(payload): Json<HackathonEventCreateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::create_hackathon_event(hackathon_id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons/{hackathon_id}/events",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "Events retrieved successfully", body = ResponseListSuccessDto<Vec<HackathonEventDto>>),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Events"
)]
pub async fn list_hackathon_events(
    Extension(state): Extension<AppState>,
    Path(hackathon_id): Path<String>,
    Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match HackathonService::list_hackathon_events(meta, hackathon_id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/v1/hackathons/events/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    request_body = HackathonEventUpdateRequestDto,
    responses(
        (status = 200, description = "Event updated successfully", body = ResponseSuccessDto<HackathonEventDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Event not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Events"
)]
pub async fn update_hackathon_event(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<HackathonEventUpdateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::update_hackathon_event(id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/hackathons/events/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Event deleted successfully", body = ResponseSuccessDto<String>),
        (status = 404, description = "Event not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Events"
)]
pub async fn delete_hackathon_event(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::delete_hackathon_event(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

// Hackathon Timeline routes
#[utoipa::path(
    post,
    path = "/v1/hackathons/{hackathon_id}/timeline",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID")
    ),
    request_body = HackathonTimelineCreateRequestDto,
    responses(
        (status = 201, description = "Timeline created successfully", body = ResponseSuccessDto<HackathonTimelineDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Timeline"
)]
pub async fn create_hackathon_timeline(
    Extension(state): Extension<AppState>,
    Path(hackathon_id): Path<String>,
    Json(payload): Json<HackathonTimelineCreateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::create_hackathon_timeline(hackathon_id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons/{hackathon_id}/timeline",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "Timeline retrieved successfully", body = ResponseListSuccessDto<Vec<HackathonTimelineDto>>),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Timeline"
)]
pub async fn list_hackathon_timeline(
    Extension(state): Extension<AppState>,
    Path(hackathon_id): Path<String>,
    Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match HackathonService::list_hackathon_timeline(meta, hackathon_id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/v1/hackathons/timeline/{id}",
    params(
        ("id" = String, Path, description = "Timeline ID")
    ),
    request_body = HackathonTimelineUpdateRequestDto,
    responses(
        (status = 200, description = "Timeline updated successfully", body = ResponseSuccessDto<HackathonTimelineDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Timeline not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Timeline"
)]
pub async fn update_hackathon_timeline(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<HackathonTimelineUpdateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::update_hackathon_timeline(id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/hackathons/timeline/{id}",
    params(
        ("id" = String, Path, description = "Timeline ID")
    ),
    responses(
        (status = 200, description = "Timeline deleted successfully", body = ResponseSuccessDto<String>),
        (status = 404, description = "Timeline not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Timeline"
)]
pub async fn delete_hackathon_timeline(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::delete_hackathon_timeline(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

// Hackathon Submissions routes
#[utoipa::path(
    post,
    path = "/v1/hackathons/{hackathon_id}/teams/{team_id}/submissions",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("team_id" = String, Path, description = "Team ID")
    ),
    request_body = HackathonSubmissionCreateRequestDto,
    responses(
        (status = 201, description = "Submission created successfully", body = ResponseSuccessDto<HackathonSubmissionDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Hackathon not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn create_hackathon_submission(
    Extension(state): Extension<AppState>,
    Path((hackathon_id, team_id)): Path<(String, String)>,
    Json(payload): Json<HackathonSubmissionCreateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::create_hackathon_submission(hackathon_id, team_id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::CREATED, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons/{hackathon_id}/submissions",
    params(
        ("hackathon_id" = String, Path, description = "Hackathon ID"),
        ("page" = Option<i64>, Query, description = "Page number"),
        ("per_page" = Option<i64>, Query, description = "Items per page"),
        ("search" = Option<String>, Query, description = "Search keyword"),
        ("sort_by" = Option<String>, Query, description = "Sort by field"),
        ("order" = Option<String>, Query, description = "Order ASC or DESC"),
        ("filter" = Option<String>, Query, description = "Filter value"),
        ("filter_by" = Option<String>, Query, description = "Field to filter by"),
    ),
    responses(
        (status = 200, description = "Submissions retrieved successfully", body = ResponseListSuccessDto<Vec<HackathonSubmissionDto>>),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn list_hackathon_submissions(
    Extension(state): Extension<AppState>,
    Path(hackathon_id): Path<String>,
    Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
    match HackathonService::list_hackathon_submissions(meta, hackathon_id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    get,
    path = "/v1/hackathons/submissions/{id}",
    params(
        ("id" = String, Path, description = "Submission ID")
    ),
    responses(
        (status = 200, description = "Submission retrieved successfully", body = ResponseSuccessDto<HackathonSubmissionDto>),
        (status = 404, description = "Submission not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn get_hackathon_submission(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::get_hackathon_submission(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    put,
    path = "/v1/hackathons/submissions/{id}",
    params(
        ("id" = String, Path, description = "Submission ID")
    ),
    request_body = HackathonSubmissionUpdateRequestDto,
    responses(
        (status = 200, description = "Submission updated successfully", body = ResponseSuccessDto<HackathonSubmissionDto>),
        (status = 400, description = "Bad request", body = ErrorDto),
        (status = 404, description = "Submission not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn update_hackathon_submission(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<HackathonSubmissionUpdateRequestDto>,
) -> impl IntoResponse {
    match HackathonService::update_hackathon_submission(id, payload, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    post,
    path = "/v1/hackathons/submissions/{id}/submit",
    params(
        ("id" = String, Path, description = "Submission ID")
    ),
    responses(
        (status = 200, description = "Submission submitted successfully", body = ResponseSuccessDto<HackathonSubmissionDto>),
        (status = 404, description = "Submission not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn submit_hackathon_submission(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::submit_hackathon_submission(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

#[utoipa::path(
    delete,
    path = "/v1/hackathons/submissions/{id}",
    params(
        ("id" = String, Path, description = "Submission ID")
    ),
    responses(
        (status = 200, description = "Submission deleted successfully", body = ResponseSuccessDto<String>),
        (status = 404, description = "Submission not found", body = ErrorDto),
        (status = 500, description = "Internal server error", body = ErrorDto)
    ),
    tag = "Hackathon Submissions"
)]
pub async fn delete_hackathon_submission(
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match HackathonService::delete_hackathon_submission(id, &state).await {
        Ok(response) => (axum::http::StatusCode::OK, Json(response)).into_response(),
        Err(error) => (StatusCode::from_u16(error.status).unwrap(), Json(error)).into_response(),
    }
}

pub fn hackathon_routes() -> Router {
    Router::new()
            // Hackathon routes
            .route("/", post(create_hackathon))
    .route("/{id}", put(update_hackathon))
        .route("/{id}", delete(delete_hackathon))

        // Hackathon Events routes
        .route("/{hackathon_id}/events", post(create_hackathon_event))
        .route("/{hackathon_id}/events", get(list_hackathon_events))
        .route("/events/{id}", put(update_hackathon_event))
        .route("/events/{id}", delete(delete_hackathon_event))

        // Hackathon Timeline routes
        .route("/{hackathon_id}/timeline", post(create_hackathon_timeline))
        .route("/{hackathon_id}/timeline", get(list_hackathon_timeline))
        .route("/timeline/{id}", put(update_hackathon_timeline))
        .route("/timeline/{id}", delete(delete_hackathon_timeline))

        // Hackathon Submissions routes
        .route("/{hackathon_id}/teams/{team_id}/submissions", post(create_hackathon_submission))
        .route("/{hackathon_id}/submissions", get(list_hackathon_submissions))
        .route("/submissions/{id}", get(get_hackathon_submission))
        .route("/submissions/{id}", put(update_hackathon_submission))
        .route("/submissions/{id}/submit", post(submit_hackathon_submission))
        .route("/submissions/{id}", delete(delete_hackathon_submission))
}