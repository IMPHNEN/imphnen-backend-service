use surrealdb::{engine::remote::ws::Client, method::Query};

pub fn bind_filter_value(
	query: Query<'_, Client>,
	val: String,
) -> Query<'_, Client> {
	query.bind(("filter", val)) // langsung string aja, udah cukup
}
