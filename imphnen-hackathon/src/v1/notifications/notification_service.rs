use super::notification_dto::{
    DeleteNotificationResponseDto, MarkAllAsReadResponseDto, MarkAsReadResponseDto,
    NotificationDto, NotificationListQueryDto, NotificationListResponseDto,
    UnreadCountResponseDto,
};
use super::notification_repository::Repository;
use super::notification_schema::NotificationSchema;
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;
use axum::body::Body;
use imphnen_entities::common_dto::ResponseSuccessDto;
use imphnen_libs::AppState;
use imphnen_utils::{
    extract_id, make_thing, response_format::success_response, validator::validate_request,
};

pub struct Service<'a> {
    state: &'a AppState,
}

impl<'a> Service<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn get_notifications(
        &self,
        user_email: &str,
        query: NotificationListQueryDto,
    ) -> Response<Body> {
        if let Err((status, message)) = validate_request(&query) {
            return (status, message).into_response();
        }

        let user_id = make_thing("users", user_email);
        let repository = Repository::new(self.state);

        let notifications_result = repository
            .query_user_notifications(
                &user_id,
                query.is_read,
                query.notification_type.clone(),
                query.page,
                query.page_size,
            )
            .await;

        let notifications = match notifications_result {
            Ok(notifs) => notifs,
            Err(err) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
            }
        };

        let total_result = repository
            .count_user_notifications(&user_id, query.is_read, query.notification_type)
            .await;

        let total = match total_result {
            Ok(count) => count,
            Err(err) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
            }
        };

        let unread_count_result = repository.count_unread_notifications(&user_id).await;

        let unread_count = match unread_count_result {
            Ok(count) => count,
            Err(err) => {
                return (StatusCode::INTERNAL_SERVER_ERROR, err).into_response();
            }
        };

        let notification_dtos: Vec<NotificationDto> = notifications
            .into_iter()
            .map(|n| NotificationDto {
                id: extract_id(&n.id),
                notification_type: format!("{:?}", n.notification_type),
                title: n.title,
                message: n.message,
                is_read: n.is_read,
                created_at: n.created_at.to_rfc3339(),
                read_at: n.read_at.map(|dt| dt.to_rfc3339()),
                related_id: n.related_id.map(|id| extract_id(&id)),
                action_url: n.action_url,
            })
            .collect();

        let response = NotificationListResponseDto {
            notifications: notification_dtos,
            total,
            unread_count,
            page: query.page,
            page_size: query.page_size,
        };

        success_response(ResponseSuccessDto { data: response })
    }

    pub async fn mark_as_read(
        &self,
        user_email: &str,
        notification_id: &str,
    ) -> Response<Body> {
        let user_id = make_thing("users", user_email);
        let notif_id = make_thing("notifications", notification_id);
        let repository = Repository::new(self.state);

        let notification_result = repository.query_notification_by_id(&notif_id).await;

        let mut notification = match notification_result {
            Ok(notif) => notif,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    "Notification not found".to_string(),
                )
                    .into_response();
            }
        };

        // Verify ownership
        if notification.user_id != user_id {
            return (
                StatusCode::FORBIDDEN,
                "You don't have permission to access this notification".to_string(),
            )
                .into_response();
        }

        if notification.is_read {
            return (
                StatusCode::BAD_REQUEST,
                "Notification is already marked as read".to_string(),
            )
                .into_response();
        }

        notification.mark_as_read();

        match repository.update_notification(&notif_id, notification.clone()).await {
            Ok(updated) => {
                let response = MarkAsReadResponseDto {
                    id: extract_id(&updated.id),
                    is_read: updated.is_read,
                    read_at: updated.read_at.unwrap().to_rfc3339(),
                    message: "Notification marked as read".to_string(),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        }
    }

    pub async fn mark_all_as_read(&self, user_email: &str) -> Response<Body> {
        let user_id = make_thing("users", user_email);
        let repository = Repository::new(self.state);

        match repository.mark_all_as_read(&user_id).await {
            Ok(count) => {
                let response = MarkAllAsReadResponseDto {
                    updated_count: count,
                    message: format!("{} notification(s) marked as read", count),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        }
    }

    pub async fn delete_notification(
        &self,
        user_email: &str,
        notification_id: &str,
    ) -> Response<Body> {
        let user_id = make_thing("users", user_email);
        let notif_id = make_thing("notifications", notification_id);
        let repository = Repository::new(self.state);

        let notification_result = repository.query_notification_by_id(&notif_id).await;

        let notification = match notification_result {
            Ok(notif) => notif,
            Err(_) => {
                return (
                    StatusCode::NOT_FOUND,
                    "Notification not found".to_string(),
                )
                    .into_response();
            }
        };

        // Verify ownership
        if notification.user_id != user_id {
            return (
                StatusCode::FORBIDDEN,
                "You don't have permission to delete this notification".to_string(),
            )
                .into_response();
        }

        match repository.delete_notification(&notif_id).await {
            Ok(_) => {
                let response = DeleteNotificationResponseDto {
                    id: notification_id.to_string(),
                    message: "Notification deleted successfully".to_string(),
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        }
    }

    pub async fn get_unread_count(&self, user_email: &str) -> Response<Body> {
        let user_id = make_thing("users", user_email);
        let repository = Repository::new(self.state);

        match repository.count_unread_notifications(&user_id).await {
            Ok(count) => {
                let response = UnreadCountResponseDto {
                    unread_count: count,
                };

                success_response(ResponseSuccessDto { data: response })
            }
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err).into_response(),
        }
    }
}
