use tracing::{info};
use chrono::{DateTime, Utc};

/// Returns the current UTC date/time as an RFC3339 string.
pub fn get_iso_date() -> String {
    info!("get_iso_date called");
    let now: DateTime<Utc> = Utc::now();
    let date_str = now.to_rfc3339();
    info!(date_str = %date_str, "get_iso_date returning RFC3339 date string");
    date_str
}
