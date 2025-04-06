use surrealdb::sql::Thing;

pub fn make_thing(table: &str, id: &str) -> Thing {
	Thing::from((table, id))
}
