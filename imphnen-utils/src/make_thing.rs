use surrealdb::sql::Thing;

pub fn make_thing(table: &str, id: &str) -> Thing {
	Thing::from((table, id))
}

pub fn make_thing_str(table: &str, id: &str) -> String {
	format!("{}:⟨{}⟩", table, id)
}
