use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthOtpSchema {
	pub otp: u32,
	pub hash: String,
	pub expires_at: DateTime<Utc>,
}
