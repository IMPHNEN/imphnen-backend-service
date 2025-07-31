use tracing::{info, error};
use anyhow::{Result, bail};
use surrealdb::sql::Thing;

/// Extracts the table and id from a Thing, returning (&str, &str).
pub fn get_id(thing: &Thing) -> Result<(&str, &str)> {
    info!(?thing, "get_id called with argument");
    let table = thing.tb.as_str();
    let id = match &thing.id {
        surrealdb::sql::Id::String(s) => {
            info!(id = %s, "ID extracted as string in get_id");
            s.as_str()
        }
        other => {
            error!(?other, "Unsupported ID type in get_id");
            bail!("Unsupported ID type");
        }
    };
    info!(table = %table, id = %id, "get_id returning table and id");
    Ok((table, id))
}

/// Extracts the raw id string from a Thing.
pub fn extract_id(thing: &Thing) -> String {
    info!(?thing, "extract_id called with argument");
    let raw_id = thing.id.to_raw();
    info!(raw_id = %raw_id, "extract_id returning raw id string");
    raw_id
}
