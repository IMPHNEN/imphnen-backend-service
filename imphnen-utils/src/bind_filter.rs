use surrealdb::engine::any;
use surrealdb::method::Query;

pub fn bind_filter_value(
	query: Query<'_, any::Any>,
	val: String,
) -> Query<'_, any::Any> {
	query.bind(("filter", val))
}
