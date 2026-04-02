pub mod csrf_token;
pub mod pagination;
pub mod errors;
pub mod extract_email;
pub mod extract_ip;
pub mod generate_date;
pub mod generate_otp;
pub mod logger;
pub mod response_format;
pub mod sanitization;

// Re-export commonly used functions
pub use extract_email::{extract_email, extract_email_async};
pub use extract_ip::extract_real_ip;
pub use generate_date::get_iso_date;
pub use response_format::{ApiSuccess, ApiCreated, ApiPaginated, ApiMessage};
pub use sanitization::{sanitize_html, sanitize_dangerous_patterns, sanitize_filename, sanitize_user_text, normalize_whitespace, sanitize_email, sanitize_url};
pub use errors::{AppError, Result};
