use imphnen_utils::errors::AppError;
use serde_json::{json, Value};

pub struct SupabaseClient {
    pub base_url: String,
    pub anon_key: String,
    pub service_role_key: String,
    pub storage_bucket: String,
    client: reqwest::Client,
}

impl SupabaseClient {
    pub fn new(base_url: String, anon_key: String, service_role_key: String, storage_bucket: String) -> Self {
        Self {
            base_url,
            anon_key,
            service_role_key,
            storage_bucket,
            client: reqwest::Client::new(),
        }
    }

    pub async fn signup(&self, email: &str, password: &str, fullname: &str, frontend_url: &str) -> Result<Value, AppError> {
        let resp = self.client
            .post(format!("{}/auth/v1/signup", self.base_url))
            .header("apikey", &self.anon_key)
            .json(&json!({
                "email": email,
                "password": password,
                "options": {
                    "data": { "fullname": fullname },
                    "emailRedirectTo": format!("{}/auth/callback", frontend_url)
                }
            }))
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::BadRequestError("Signup failed. Email may already be registered.".to_string()));
        }
        resp.json::<Value>().await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<Value, AppError> {
        let resp = self.client
            .post(format!("{}/auth/v1/token?grant_type=password", self.base_url))
            .header("apikey", &self.anon_key)
            .json(&json!({ "email": email, "password": password }))
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::AuthenticationError("Invalid email or password".to_string()));
        }
        resp.json::<Value>().await.map_err(|e| AppError::InternalServerError(e.to_string()))
    }

    pub async fn recover_password(&self, email: &str, frontend_url: &str) -> Result<(), AppError> {
        let _ = self.client
            .post(format!("{}/auth/v1/recover", self.base_url))
            .header("apikey", &self.anon_key)
            .json(&json!({
                "email": email,
                "redirectTo": format!("{}/auth/callback", frontend_url)
            }))
            .send()
            .await;
        Ok(())
    }

    pub async fn update_password(&self, access_token: &str, new_password: &str) -> Result<(), AppError> {
        let resp = self.client
            .put(format!("{}/auth/v1/user", self.base_url))
            .header("apikey", &self.anon_key)
            .header("Authorization", format!("Bearer {}", access_token))
            .json(&json!({ "password": new_password }))
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(AppError::BadRequestError("Password reset failed. Link may have expired.".to_string()));
        }
        Ok(())
    }

    pub async fn upload_file(&self, path: &str, content_type: &str, data: &[u8]) -> Result<String, AppError> {
        let resp = self.client
            .post(format!("{}/storage/v1/object/{}/{}", self.base_url, self.storage_bucket, path))
            .header("apikey", &self.service_role_key)
            .header("Authorization", format!("Bearer {}", self.service_role_key))
            .header("Content-Type", content_type)
            .body(data.to_vec())
            .send()
            .await
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        if !resp.status().is_success() {
            let err = resp.text().await.unwrap_or_default();
            return Err(AppError::InternalServerError(format!("Upload failed: {}", err)));
        }

        Ok(format!("{}/storage/v1/object/public/{}/{}", self.base_url, self.storage_bucket, path))
    }
}
