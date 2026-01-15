pub mod csrf_token;
pub mod errors;
pub mod extract_email;
pub mod extract_ip;
pub mod generate_date;
pub mod generate_otp;
pub mod logger;
pub mod migration_validation_errors;
pub mod response_format;
pub mod sanitization;
pub mod validator;

// Re-export commonly used functions
pub use extract_email::{extract_email, extract_email_async};
pub use extract_ip::extract_real_ip;
pub use generate_date::get_iso_date;
pub use response_format::{success_response, success_created_response, success_list_response, common_response, error_response};
pub use validator::validate_request;
pub use sanitization::{sanitize_html, sanitize_dangerous_patterns, sanitize_filename, sanitize_user_text, normalize_whitespace, sanitize_email, sanitize_url};
pub use errors::{AppError, Result};

// Add missing utility functions for database operations
pub fn make_thing(_resource: &str, id: &str) -> String {
    id.to_string()
}

pub fn make_thing_from_enum(_resource_enum: &str, id: &str) -> String {
    id.to_string()
}

/// A compatibility wrapper for SurrealDB's `Thing` type
/// Many test helpers and older modules use `Thing::from((resource, id))`.
/// Provide a small compatibility struct that can be constructed like that and
/// converted to a String so code will compile with PostgreSQL-backed storage.
#[derive(Clone, Debug)]
pub struct Thing(pub String);

impl Thing {
    pub fn from((_resource, id): (&str, &str)) -> Self {
        Thing(id.to_string())
    }
}

impl From<Thing> for String {
    fn from(t: Thing) -> Self {
        t.0
    }
}

use sea_orm::{DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait};
use imphnen_entities::seaorm::auth::users::Entity as UsersEntity;

/// Get user ID from email address
pub async fn get_user_id_from_email(email: &str, db: &DatabaseConnection) -> Result<String> {
    let user = UsersEntity::find()
        .filter(imphnen_entities::seaorm::auth::users::Column::Email.eq(email))
        .one(db)
        .await?;
    
    match user {
        Some(u) => Ok(u.id.to_string()),
        None => Err(AppError::NotFoundError("User not found".to_string())),
    }
}
