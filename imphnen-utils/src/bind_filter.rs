use surrealdb::method::Query;
use surrealdb::engine::any;

pub fn bind_filter_value(
	query: Query<'_, any::Any>,
	val: String,
) -> Query<'_, any::Any> {
	query.bind(("filter", val)) // langsung string aja, udah cukup
}
