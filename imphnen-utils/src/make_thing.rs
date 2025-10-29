use surrealdb::sql::Thing;
use std::fmt::Display;

pub fn make_thing(table: &str, id: &str) -> Thing {
	Thing::from((table, id))
}

pub fn make_thing_from_enum<T: Display>(table: T, id: &str) -> Thing {
	Thing::from((table.to_string().as_str(), id))
}

pub fn make_thing_str(table: &str, id: &str) -> String {
	format!("{table}:⟨{id}⟩")
}
