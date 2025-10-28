use super::notification_dto::{
    DeleteNotificationResponseDto, MarkAllAsReadResponseDto, MarkAsReadResponseDto, NotificationListQueryDto, NotificationListResponseDto,
    UnreadCountResponseDto,
};
use super::notification_service::Service;
use axum::{
    extract::{Extension, Path, Query},
    http::{HeaderMap, Response, StatusCode},
    routing::{delete, get, put},
    Router, body::Body,
};
use imphnen_libs::AppState;
use imphnen_utils::{extract_email::extract_email, response_format::common_response};

/// Get user's notifications with optional filtering
#[utoipa::path(
    get,
    path = "/v1/notifications",
    tags = ["notifications"],
    params(
        ("page_size" = Option<usize>, Query, description = "Number of notifications per page (1-100, default: 20)"),
        ("page" = Option<usize>, Query, description = "Page number (min: 1, default: 1)"),
        ("is_read" = Option<bool>, Query, description = "Filter by read status"),
        ("notification_type" = Option<String>, Query, description = "Filter by notification type"),
    ),
    responses(
        (status = 200, description = "Successfully retrieved notifications", body = NotificationListResponseDto),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn get_notifications_handler(
    headers: HeaderMap,
    Query(query): Query<NotificationListQueryDto>,
    Extension(state): Extension<AppState>,
) -> Response<Body> {
    match extract_email(&headers) {
        Some(email) => {
            let service = Service::new(&state);
            service.get_notifications(&email, query).await
        }
        None => common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}

/// Mark a notification as read
#[utoipa::path(
    put,
    path = "/v1/notifications/{id}/read",
    tags = ["notifications"],
    params(
        ("id" = String, Path, description = "Notification ID"),
    ),
    responses(
        (status = 200, description = "Successfully marked notification as read", body = MarkAsReadResponseDto),
        (status = 400, description = "Notification already marked as read"),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
        (status = 403, description = "Forbidden - Not the notification owner"),
        (status = 404, description = "Notification not found"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn mark_as_read_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    Extension(state): Extension<AppState>,
) -> Response<Body> {
    match extract_email(&headers) {
        Some(email) => {
            let service = Service::new(&state);
            service.mark_as_read(&email, &id).await
        }
        None => common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}

/// Mark all notifications as read
#[utoipa::path(
    put,
    path = "/v1/notifications/read-all",
    tags = ["notifications"],
    responses(
        (status = 200, description = "Successfully marked all notifications as read", body = MarkAllAsReadResponseDto),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn mark_all_as_read_handler(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
) -> Response<Body> {
    match extract_email(&headers) {
        Some(email) => {
            let service = Service::new(&state);
            service.mark_all_as_read(&email).await
        }
        None => common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}

/// Delete a notification
#[utoipa::path(
    delete,
    path = "/v1/notifications/{id}",
    tags = ["notifications"],
    params(
        ("id" = String, Path, description = "Notification ID"),
    ),
    responses(
        (status = 200, description = "Successfully deleted notification", body = DeleteNotificationResponseDto),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
        (status = 403, description = "Forbidden - Not the notification owner"),
        (status = 404, description = "Notification not found"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn delete_notification_handler(
    headers: HeaderMap,
    Path(id): Path<String>,
    Extension(state): Extension<AppState>,
) -> Response<Body> {
    match extract_email(&headers) {
        Some(email) => {
            let service = Service::new(&state);
            service.delete_notification(&email, &id).await
        }
        None => common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}

/// Get unread notifications count
#[utoipa::path(
    get,
    path = "/v1/notifications/unread/count",
    tags = ["notifications"],
    responses(
        (status = 200, description = "Successfully retrieved unread count", body = UnreadCountResponseDto),
        (status = 401, description = "Unauthorized - Invalid or missing token"),
    ),
    security(
        ("bearer" = [])
    )
)]
pub async fn get_unread_count_handler(
    headers: HeaderMap,
    Extension(state): Extension<AppState>,
) -> Response<Body> {
    match extract_email(&headers) {
        Some(email) => {
            let service = Service::new(&state);
            service.get_unread_count(&email).await
        }
        None => common_response(StatusCode::UNAUTHORIZED, "Unauthorized"),
    }
}

pub fn notifications_router() -> Router {
    Router::new()
        .route("/notifications", get(get_notifications_handler))
        .route("/notifications/{id}/read", put(mark_as_read_handler))
        .route("/notifications/read-all", put(mark_all_as_read_handler))
        .route("/notifications/{id}", delete(delete_notification_handler))
        .route("/notifications/unread/count", get(get_unread_count_handler))
}
