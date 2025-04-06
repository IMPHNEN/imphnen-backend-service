use anyhow::{bail, Result};
use surrealdb::sql::Thing;

pub fn get_id(thing: &Thing) -> Result<(&str, &str)> {
	let table = thing.tb.as_str();
	let id = match &thing.id {
		surrealdb::sql::Id::String(s) => s.as_str(),
		_ => bail!("Unsupported ID type"),
	};
	Ok((table, id))
}

pub fn extract_id(thing: &Thing) -> String {
	let id = thing.id.to_raw();
	id
}
