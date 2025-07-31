use tracing::{info};
use surrealdb::engine::any;
use surrealdb::method::Query;

/// Binds a filter value to the query under the key "filter".
pub fn bind_filter_value(
	query: Query<'_, any::Any>,
	val: String,
) -> Query<'_, any::Any> {
	info!(?val, "bind_filter_value called with arguments");
	let result = query.bind(("filter", val.clone()));
	info!(?val, "bind_filter_value returning query with bound filter");
	result
}
