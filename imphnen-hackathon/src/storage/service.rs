use std::sync::Arc;
use base64::Engine;
use chrono::Utc;
use uuid::Uuid;
use imphnen_utils::errors::AppError;
use crate::common::supabase_client::SupabaseClient;

pub struct StorageService {
    supabase: Arc<SupabaseClient>,
}

impl StorageService {
    pub fn new(supabase: Arc<SupabaseClient>) -> Self { Self { supabase } }

    pub async fn upload(&self, folder: &str, user_id: Uuid, filename: &str, content_type: &str, data_base64: &str) -> Result<String, AppError> {
        let ext = filename.rsplit('.').next().unwrap_or("bin");
        let path = format!("{}/{}-{}.{}", folder, user_id, Utc::now().timestamp_millis(), ext);
        let data = base64::engine::general_purpose::STANDARD.decode(data_base64)
            .map_err(|_| AppError::BadRequestError("Invalid base64 data".to_string()))?;
        self.supabase.upload_file(&path, content_type, &data).await
    }
}
