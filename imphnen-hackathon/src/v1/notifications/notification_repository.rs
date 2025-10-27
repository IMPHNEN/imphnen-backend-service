use crate::v1::notifications::notification_schema::NotificationSchema;
use imphnen_libs::AppState;
use imphnen_utils::{get_id, make_thing};
use surrealdb::sql::Thing;

pub struct Repository<'a> {
    state: &'a AppState,
}

impl<'a> Repository<'a> {
    pub fn new(state: &'a AppState) -> Self {
        Self { state }
    }

    pub async fn query_user_notifications(
        &self,
        user_id: &Thing,
        is_read: Option<bool>,
        notification_type: Option<String>,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<NotificationSchema>, String> {
        let db = &self.state.surrealdb_ws;
        let offset = (page - 1) * page_size;

        let mut query = "SELECT * FROM notifications WHERE user_id = $user_id ".to_string();

        if let Some(is_read_val) = is_read {
            query.push_str(&format!(" AND is_read = {} ", is_read_val));
        }

        if let Some(ref notif_type) = notification_type {
            query.push_str(&format!(" AND notification_type = '{}' ", notif_type));
        }

        query.push_str(&format!(
            " ORDER BY created_at DESC LIMIT {} START {} ",
            page_size, offset
        ));

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(&query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Query failed: {}", e))?;
        let notifications: Vec<NotificationSchema> = result.take(0).map_err(|e| format!("Failed to parse results: {}", e))?;
        Ok(notifications)
    }

    pub async fn count_user_notifications(
        &self,
        user_id: &Thing,
        is_read: Option<bool>,
        notification_type: Option<String>,
    ) -> Result<usize, String> {
        let db = &self.state.surrealdb_ws;

        let mut query = "SELECT count() as total FROM notifications WHERE user_id = $user_id ".to_string();

        if let Some(is_read_val) = is_read {
            query.push_str(&format!(" AND is_read = {} ", is_read_val));
        }

        if let Some(ref notif_type) = notification_type {
            query.push_str(&format!(" AND notification_type = '{}' ", notif_type));
        }

        query.push_str(" GROUP ALL ");

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(&query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Query failed: {}", e))?;
        let count: Option<usize> = result.take("total").map_err(|e| format!("Failed to get count: {}", e))?;
        Ok(count.unwrap_or(0))
    }

    pub async fn query_notification_by_id(
        &self,
        notification_id: &Thing,
    ) -> Result<NotificationSchema, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(notification_id).map_err(|e| e.to_string())?;
        let notification: Option<NotificationSchema> = db
            .select(record_key)
            .await
            .map_err(|e| format!("Failed to fetch notification: {}", e))?;
        notification.ok_or("Notification not found".to_string())
    }

    pub async fn update_notification(
        &self,
        notification_id: &Thing,
        notification: NotificationSchema,
    ) -> Result<NotificationSchema, String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(notification_id).map_err(|e| e.to_string())?;
        let updated: Option<NotificationSchema> = db
            .update(record_key)
            .content(notification)
            .await
            .map_err(|e| format!("Failed to update notification: {}", e))?;
        updated.ok_or("Failed to update notification".to_string())
    }

    pub async fn mark_all_as_read(&self, user_id: &Thing) -> Result<usize, String> {
        let db = &self.state.surrealdb_ws;
        
        let query = "UPDATE notifications SET is_read = true, read_at = time::now() WHERE user_id = $user_id AND is_read = false";

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Query failed: {}", e))?;
        let updated: Vec<NotificationSchema> = result.take(0).map_err(|e| format!("Failed to parse results: {}", e))?;
        Ok(updated.len())
    }

    pub async fn delete_notification(&self, notification_id: &Thing) -> Result<(), String> {
        let db = &self.state.surrealdb_ws;
        let record_key = get_id(notification_id).map_err(|e| e.to_string())?;
        let _: Option<NotificationSchema> = db
            .delete(record_key)
            .await
            .map_err(|e| format!("Failed to delete notification: {}", e))?;
        Ok(())
    }

    pub async fn count_unread_notifications(&self, user_id: &Thing) -> Result<usize, String> {
        let db = &self.state.surrealdb_ws;

        let query = "SELECT count() as total FROM notifications WHERE user_id = $user_id AND is_read = false GROUP ALL";

        let user_id_clone = user_id.clone();
        let mut result = db
            .query(query)
            .bind(("user_id", user_id_clone))
            .await
            .map_err(|e| format!("Query failed: {}", e))?;
        let count: Option<usize> = result.take("total").map_err(|e| format!("Failed to get count: {}", e))?;
        Ok(count.unwrap_or(0))
    }
}
