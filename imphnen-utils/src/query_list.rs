use super::bind_filter_value;
use crate::{CountResult, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto};
use anyhow::{bail, Result};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::{engine::remote::ws::Client, Surreal};

pub async fn query_list_with_meta<T>(
	db: &Surreal<Client>,
	table: &str,
	meta: &MetaRequestDto,
	conditions: Vec<String>,
	custom_select: Option<String>,
) -> Result<ResponseListSuccessDto<Vec<T>>>
where
	T: DeserializeOwned + Serialize,
{
	let page = meta.page.unwrap_or(1);
	let per_page = meta.per_page.unwrap_or(10);
	if page < 1 || per_page < 1 {
		bail!("Invalid pagination: page and per_page must be greater than 0");
	}
	let start = (page - 1) * per_page;
	let sql = custom_select.unwrap_or_else(|| {
		let mut s = format!("SELECT * FROM {}", table);
		if !conditions.is_empty() {
			s.push_str(" WHERE ");
			s.push_str(&conditions.join(" AND "));
		}
		if let Some(sort_by) = &meta.sort_by {
			let order = match meta
				.order
				.clone()
				.unwrap_or_default()
				.to_uppercase()
				.as_str()
			{
				"DESC" => "DESC",
				_ => "ASC",
			};
			s.push_str(&format!(" ORDER BY {} {}", sort_by, order));
		}
		s.push_str(" LIMIT $per_page START $start");
		s
	});
	let mut query_exec = db.query(sql);
	if let Some(search) = &meta.search {
		if !search.is_empty() {
			query_exec = query_exec.bind(("search", search.clone()));
		}
	}
	if let Some(filter_val) = meta.filter.clone() {
		query_exec = bind_filter_value(query_exec, filter_val);
	}
	query_exec = query_exec
		.bind(("per_page", per_page))
		.bind(("start", start));
	let raw: Vec<T> = query_exec.await?.take(0)?;
	let mut count_sql = format!("SELECT count() FROM {}", table);
	if !conditions.is_empty() {
		count_sql.push_str(" WHERE ");
		count_sql.push_str(&conditions.join(" AND "));
	}
	let mut count_query = db.query(count_sql);
	if let Some(search) = &meta.search {
		if !search.is_empty() {
			count_query = count_query.bind(("search", search.clone()));
		}
	}
	if let Some(filter_val) = meta.filter.clone() {
		count_query = bind_filter_value(count_query, filter_val);
	}
	let count_result: Vec<CountResult> = count_query.await?.take(0)?;
	let total = count_result.first().map(|c| c.count);

	let meta = MetaResponseDto {
		page: Some(page),
		per_page: Some(per_page),
		total,
	};
	Ok(ResponseListSuccessDto {
		data: raw,
		meta: Some(meta),
	})
}
