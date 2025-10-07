use super::{
	events_dto::{
		EventsCreateRequestDto, EventsDetailItemDto, EventsListItemDto,
		EventsUpdateRequestDto,
	},
	events_service::EventsService,
};
use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::{Extension, Json, http::HeaderMap};
use imphnen_libs::{
    AppState, MessageResponseDto, MetaRequestDto, ResponseListSuccessDto,
    ResponseSuccessDto,
};
use imphnen_iam::{PermissionsEnum, permissions_guard};

#[utoipa::path(
    get,
    path = "/v1/cms/landing/events",
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
        (status = 200, description = "Get event list", body = ResponseListSuccessDto<Vec<EventsListItemDto>>)
    ),
    tag = "Events"
)]
pub async fn get_event_list(
	Extension(state): Extension<AppState>,
	Query(meta): Query<MetaRequestDto>,
) -> impl IntoResponse {
	EventsService::get_event_list(&state, meta).await
}

#[utoipa::path(
    get,
    path = "/v1/cms/landing/events/detail/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Get event by ID", body = ResponseSuccessDto<EventsDetailItemDto>)
    ),
    tag = "Events"
)]
pub async fn get_event_by_id(
	Extension(state): Extension<AppState>,
	Path(id): Path<String>,
) -> impl IntoResponse {
	EventsService::get_event_by_id(&state, id).await
}

#[utoipa::path(
    post,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/events/create",
    request_body = EventsCreateRequestDto,
    responses(
        (status = 201, description = "Create new event", body = MessageResponseDto)
    ),
    tag = "Events"
)]
pub async fn post_create_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Json(payload): Json<EventsCreateRequestDto>,
) -> impl IntoResponse {
    match permissions_guard(headers, Extension(state), vec![PermissionsEnum::Administrator]).await {
        Ok((_claims, state)) => EventsService::create_event(&state, payload).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    patch,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/events/update/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    request_body = EventsUpdateRequestDto,
    responses(
        (status = 200, description = "Update event", body = MessageResponseDto)
    ),
    tag = "Events"
)]
pub async fn patch_update_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<EventsUpdateRequestDto>,
) -> impl IntoResponse {
    match permissions_guard(headers, Extension(state), vec![PermissionsEnum::Administrator]).await {
        Ok((_claims, state)) => EventsService::update_event(&state, id, payload).await,
        Err(response) => response,
    }
}

#[utoipa::path(
    delete,
    security(
        ("Bearer" = [])
    ),
    path = "/v1/cms/landing/events/delete/{id}",
    params(
        ("id" = String, Path, description = "Event ID")
    ),
    responses(
        (status = 200, description = "Soft delete event", body = MessageResponseDto)
    ),
    tag = "Events"
)]
pub async fn delete_event(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match permissions_guard(headers, Extension(state), vec![PermissionsEnum::Administrator]).await {
        Ok((_claims, state)) => EventsService::delete_event(&state, id).await,
        Err(response) => response,
    }
}
