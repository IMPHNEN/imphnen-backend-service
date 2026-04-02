use std::env;

#[derive(Debug, Clone)]
pub struct QrConfig {
    pub jwt_secret: String,
    pub jwt_expiry_minutes: i64,
    pub refresh_expiry_days: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_url: String,
}

impl QrConfig {
    pub fn from_env() -> Self {
        Self {
            jwt_secret: env::var("QR_JWT_SECRET").expect("QR_JWT_SECRET must be set"),
            jwt_expiry_minutes: env::var("QR_JWT_EXPIRY_MINUTES")
                .unwrap_or_else(|_| "15".to_string())
                .parse()
                .unwrap_or(15),
            refresh_expiry_days: env::var("QR_JWT_REFRESH_EXPIRY_DAYS")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .unwrap_or(7),
            google_client_id: env::var("QR_GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: env::var("QR_GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            google_redirect_url: env::var("QR_GOOGLE_REDIRECT_URL").unwrap_or_default(),
        }
    }
}
