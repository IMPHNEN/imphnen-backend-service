use chrono::{DateTime, Utc};
use tracing::info;

pub fn get_iso_date() -> String {
	info!("get_iso_date called");
	let now: DateTime<Utc> = Utc::now();
	let date_str = now.to_rfc3339();
	info!(date_str = %date_str, "get_iso_date returning RFC3339 date string");
	date_str
}

#[cfg(test)]
mod tests {
	use super::*;
	use chrono::DateTime;

	#[test]
	fn test_get_iso_date() {
		let date_str = get_iso_date();
		let parsed = DateTime::parse_from_rfc3339(&date_str);
		assert!(parsed.is_ok());
		let now = Utc::now();
		let parsed = parsed.unwrap().with_timezone(&Utc);
		let diff = (now - parsed).num_milliseconds().abs();
		assert!(diff < 1000);
	}
}
