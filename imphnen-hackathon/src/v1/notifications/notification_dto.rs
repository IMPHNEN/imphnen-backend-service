use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NotificationDto {
    pub id: String,
    pub notification_type: String,
    pub title: String,
    pub message: String,
    pub is_read: bool,
    pub created_at: String,
    pub read_at: Option<String>,
    pub related_id: Option<String>,
    pub action_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct NotificationListResponseDto {
    pub notifications: Vec<NotificationDto>,
    pub total: usize,
    pub unread_count: usize,
    pub page: usize,
    pub page_size: usize,
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct NotificationListQueryDto {
    #[validate(range(min = 1, max = 100))]
    #[serde(default = "default_page_size")]
    pub page_size: usize,
    
    #[validate(range(min = 1))]
    #[serde(default = "default_page")]
    pub page: usize,
    
    pub is_read: Option<bool>,
    pub notification_type: Option<String>,
}

fn default_page_size() -> usize {
    20
}

fn default_page() -> usize {
    1
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarkAsReadResponseDto {
    pub id: String,
    pub is_read: bool,
    pub read_at: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct MarkAllAsReadResponseDto {
    pub updated_count: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct DeleteNotificationResponseDto {
    pub id: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UnreadCountResponseDto {
    pub unread_count: usize,
}
