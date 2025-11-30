//! Utility functions for SeaORM entities
//! Provides helper functions for UUID generation, timestamp handling, and resource management

use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::types::{PgTimestamp, PgUuid};

/// Generate a new UUID for entity IDs
/// Uses cryptographically secure random UUID version 4
pub fn generate_uuid() -> Uuid {
    Uuid::new_v4()
}

/// Generate a new timestamp for entity timestamps
/// Uses UTC timezone with millisecond precision
pub fn generate_timestamp() -> PgTimestamp {
    PgTimestamp(DateTime::from_timestamp_millis(Utc::now().timestamp_millis()).unwrap())
}

/// Convert a string to PgUuid
/// Returns Result<PgUuid, String> with error message on failure
pub fn string_to_uuid(uuid_str: &str) -> Result<PgUuid, String> {
    Uuid::parse_str(uuid_str)
        .map(PgUuid)
        .map_err(|e| format!("Invalid UUID format: {e}"))
}

/// Convert PgUuid to string representation
pub fn uuid_to_string(uuid: &uuid::Uuid) -> String {
    uuid.to_string()
}

/// Get current timestamp as DateTime<Utc>
pub fn current_timestamp() -> DateTime<Utc> {
    Utc::now()
}

/// Format timestamp for display
pub fn format_timestamp(timestamp: &PgTimestamp) -> String {
    timestamp.0.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Create a soft delete timestamp
pub fn create_deleted_at() -> Option<DateTime<Utc>> {
    Some(current_timestamp())
}

/// Remove soft delete timestamp
pub fn remove_deleted_at() -> Option<PgTimestamp> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_uuid() {
        let uuid1 = generate_uuid();
        let uuid2 = generate_uuid();
        assert_ne!(uuid1, uuid2);
        assert!(Uuid::parse_str(&uuid_to_string(&uuid1)).is_ok());
    }

    #[test]
    fn test_generate_timestamp() {
        let ts1 = generate_timestamp();
        let ts2 = generate_timestamp();
        // Timestamps should be close to each other
        let diff = ts2.0.signed_duration_since(ts1.0).num_milliseconds();
        assert!(diff >= 0);
        assert!(diff < 1000); // Should be within 1 second
    }

    #[test]
    fn test_string_to_uuid() {
        let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
        let result = string_to_uuid(uuid_str);
        assert!(result.is_ok());
        let uuid = result.unwrap();
        // `uuid` is a `PgUuid`; convert to `Uuid` before comparing string representation
        let uuid_plain: uuid::Uuid = uuid.into();
        assert_eq!(uuid_to_string(&uuid_plain), uuid_str);

        let invalid_uuid = "invalid-uuid";
        let result = string_to_uuid(invalid_uuid);
        assert!(result.is_err());
    }
}