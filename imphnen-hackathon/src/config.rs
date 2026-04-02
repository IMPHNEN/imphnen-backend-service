use std::env;

#[derive(Debug, Clone)]
pub struct HackathonConfig {
    pub supabase_url: String,
    pub supabase_anon_key: String,
    pub supabase_service_role_key: String,
    pub jwt_secret: String,
    pub jwt_expiry_hours: i64,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_url: String,
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_email: String,
    pub storage_bucket: String,
    pub frontend_url: String,
}

impl HackathonConfig {
    pub fn from_env() -> Self {
        Self {
            supabase_url: env::var("HACKATHON_SUPABASE_URL").unwrap_or_default(),
            supabase_anon_key: env::var("HACKATHON_SUPABASE_ANON_KEY").unwrap_or_default(),
            supabase_service_role_key: env::var("HACKATHON_SUPABASE_SERVICE_ROLE_KEY").unwrap_or_default(),
            jwt_secret: env::var("HACKATHON_JWT_SECRET").expect("HACKATHON_JWT_SECRET must be set"),
            jwt_expiry_hours: env::var("HACKATHON_JWT_EXPIRY_HOURS")
                .unwrap_or_else(|_| "168".to_string())
                .parse()
                .unwrap_or(168),
            github_client_id: env::var("HACKATHON_GITHUB_CLIENT_ID").unwrap_or_default(),
            github_client_secret: env::var("HACKATHON_GITHUB_CLIENT_SECRET").unwrap_or_default(),
            github_redirect_url: env::var("HACKATHON_GITHUB_REDIRECT_URL").unwrap_or_default(),
            smtp_host: env::var("HACKATHON_SMTP_HOST").unwrap_or_default(),
            smtp_user: env::var("HACKATHON_SMTP_USER").unwrap_or_default(),
            smtp_password: env::var("HACKATHON_SMTP_PASSWORD").unwrap_or_default(),
            from_email: env::var("HACKATHON_FROM_EMAIL").unwrap_or_default(),
            storage_bucket: env::var("HACKATHON_STORAGE_BUCKET")
                .unwrap_or_else(|_| "hackathon-uploads".to_string()),
            frontend_url: env::var("HACKATHON_FRONTEND_URL")
                .unwrap_or_else(|_| "https://hackathon.imphnen.dev".to_string()),
        }
    }
}
