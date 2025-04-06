use surrealdb::{engine::remote::ws::Client, method::Query};

pub fn bind_filter_value(
	query: Query<'_, Client>,
	val: String,
) -> Query<'_, Client> {
	if let Ok(b) = val.parse::<bool>() {
		query.bind(("filter", b))
	} else if let Ok(i) = val.parse::<i64>() {
		query.bind(("filter", i))
	} else {
		query.bind(("filter", val))
	}
}
