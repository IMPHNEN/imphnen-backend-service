use super::{ListQueryBuilder, bind_filter_value};
use crate::{CountResult, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto};
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn query_list_with_meta<T>(
	db: &Surreal<Client>,
	table: &str,
	meta: &MetaRequestDto,
	conditions: Vec<String>,
	custom_select: Option<String>,
	search_field: &str,
	select_fields: Option<Vec<&str>>,
) -> Result<ResponseListSuccessDto<Vec<T>>>
where
	T: DeserializeOwned + Serialize,
{
	let page = meta.page.unwrap_or(1).max(1);
	let per_page = meta.per_page.unwrap_or(10).max(1);
	let start = (page - 1) * per_page;

	let sql = custom_select.unwrap_or_else(|| {
		ListQueryBuilder::from_meta(table, meta, search_field, select_fields).build()
	});

	let mut query_exec = db.query(sql);
	if let Some(search) = &meta.search {
		if !search.is_empty() {
			query_exec = query_exec.bind(("search", search.clone()));
		}
	}
	if let Some(filter_val) = &meta.filter {
		query_exec = bind_filter_value(query_exec, filter_val.clone());
	}
	query_exec = query_exec
		.bind(("per_page", per_page))
		.bind(("start", start));

	let raw: Vec<T> = query_exec.await?.take(0)?;

	let mut count_query = db.query(format!(
		"SELECT count() FROM {} {}",
		table,
		if conditions.is_empty() {
			String::new()
		} else {
			format!("WHERE {}", conditions.join(" AND "))
		}
	));
	if let Some(search) = &meta.search {
		if !search.is_empty() {
			count_query = count_query.bind(("search", search.clone()));
		}
	}
	if let Some(filter_val) = &meta.filter {
		count_query = bind_filter_value(count_query, filter_val.clone());
	}
	let count_result: Vec<CountResult> = count_query.await?.take(0)?;
	let total = count_result.first().map(|c| c.count);

	Ok(ResponseListSuccessDto {
		data: raw,
		meta: Some(MetaResponseDto {
			page: Some(page),
			per_page: Some(per_page),
			total,
		}),
	})
}
