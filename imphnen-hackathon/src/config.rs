use std::env;

#[derive(Debug, Clone)]
pub struct HackathonConfig {
    pub smtp_host: String,
    pub smtp_user: String,
    pub smtp_password: String,
    pub from_email: String,
    pub frontend_url: String,
}

impl HackathonConfig {
    pub fn from_env() -> Self {
        Self {
            smtp_host: env::var("HACKATHON_SMTP_HOST").unwrap_or_default(),
            smtp_user: env::var("HACKATHON_SMTP_USER").unwrap_or_default(),
            smtp_password: env::var("HACKATHON_SMTP_PASSWORD").unwrap_or_default(),
            from_email: env::var("HACKATHON_FROM_EMAIL").unwrap_or_default(),
            frontend_url: env::var("HACKATHON_FRONTEND_URL")
                .unwrap_or_else(|_| "https://hackathon.imphnen.dev".to_string()),
        }
    }
}
