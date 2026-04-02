use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use imphnen_utils::errors::AppError;
use imphnen_libs::MinioService;

pub struct StorageService {
    minio: Arc<MinioService>,
}

impl StorageService {
    pub fn new(minio: Arc<MinioService>) -> Self { Self { minio } }

    pub async fn upload(&self, folder: &str, user_id: Uuid, filename: &str, content_type: &str, data_base64: &str) -> Result<String, AppError> {
        let ext = filename.rsplit('.').next().unwrap_or("bin");
        let unique_name = format!("{}-{}.{}", user_id, Utc::now().timestamp_millis(), ext);
        self.minio
            .upload_base64_file(data_base64, content_type, folder, &unique_name)
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))
    }
}
