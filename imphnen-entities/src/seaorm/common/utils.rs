use chrono::{DateTime, Utc};
use uuid::Uuid;

use super::types::{PgTimestamp, PgUuid};

pub fn generate_uuid() -> Uuid {
	Uuid::new_v4()
}

pub fn generate_timestamp() -> PgTimestamp {
	PgTimestamp(Utc::now())
}

pub fn string_to_uuid(uuid_str: &str) -> Result<PgUuid, String> {
	Uuid::parse_str(uuid_str)
		.map(PgUuid)
		.map_err(|e| format!("Invalid UUID format: {e}"))
}

pub fn uuid_to_string(uuid: &uuid::Uuid) -> String {
	uuid.to_string()
}

pub fn current_timestamp() -> DateTime<Utc> {
	Utc::now()
}

pub fn format_timestamp(timestamp: &PgTimestamp) -> String {
	timestamp.0.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn create_deleted_at() -> Option<DateTime<Utc>> {
	Some(current_timestamp())
}

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
		let diff = ts2.0.signed_duration_since(ts1.0).num_milliseconds();
		assert!(diff >= 0);
		assert!(diff < 1000);
	}

	#[test]
	fn test_string_to_uuid() {
		let uuid_str = "123e4567-e89b-12d3-a456-426614174000";
		let result = string_to_uuid(uuid_str);
		assert!(result.is_ok());
		let uuid = result.unwrap();
		let uuid_plain: uuid::Uuid = uuid.into();
		assert_eq!(uuid_to_string(&uuid_plain), uuid_str);

		let invalid_uuid = "invalid-uuid";
		let result = string_to_uuid(invalid_uuid);
		assert!(result.is_err());
	}
}
