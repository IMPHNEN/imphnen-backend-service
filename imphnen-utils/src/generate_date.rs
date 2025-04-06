use chrono::{DateTime, Utc};

pub fn get_iso_date() -> String {
	let now: DateTime<Utc> = Utc::now();
	now.to_rfc3339()
}
