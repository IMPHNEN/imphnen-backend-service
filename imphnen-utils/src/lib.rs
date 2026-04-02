pub mod csrf_token;
pub mod errors;
pub mod extract_email;
pub mod extract_ip;
pub mod generate_date;
pub mod generate_otp;
pub mod logger;
pub mod pagination;
pub mod response_format;
pub mod sanitization;

pub use errors::{AppError, Result};
pub use extract_email::{extract_email, extract_email_async};
pub use extract_ip::extract_real_ip;
pub use generate_date::get_iso_date;
pub use response_format::{ApiCreated, ApiMessage, ApiPaginated, ApiSuccess};
pub use sanitization::{
	normalize_whitespace, sanitize_dangerous_patterns, sanitize_email,
	sanitize_filename, sanitize_html, sanitize_url, sanitize_user_text,
};
