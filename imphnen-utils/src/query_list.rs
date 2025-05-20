use crate::{CountResult, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto};
use anyhow::Result;
use serde::{Serialize, de::DeserializeOwned};
use surrealdb::{Surreal, engine::remote::ws::Client};

pub struct QueryListBuilder<'a> {
	db: &'a Surreal<Client>,
	table: &'a str,
	meta: &'a MetaRequestDto,
	conditions: Vec<String>,
	search_field: String,
	select_fields: Option<Vec<&'a str>>,
	fetch_fields: Option<Vec<&'a str>>,
}

impl<'a> QueryListBuilder<'a> {
	pub fn new(
		db: &'a Surreal<Client>,
		table: &'a str,
		meta: &'a MetaRequestDto,
	) -> Self {
		Self {
			db,
			table,
			meta,
			conditions: vec![],
			search_field: "name".to_string(),
			select_fields: None,
			fetch_fields: None,
		}
	}

	pub fn search_field(mut self, field: &'a str) -> Self {
		self.search_field = field.to_string();
		self
	}

	pub fn select_fields(mut self, fields: Vec<&'a str>) -> Self {
		self.select_fields = Some(fields);
		self
	}

	pub fn fetch_fields(mut self, fields: Vec<&'a str>) -> Self {
		self.fetch_fields = Some(fields);
		self
	}

	pub fn with_condition(mut self, condition: &str) -> Self {
		self.conditions.push(condition.to_string());
		self
	}

	pub async fn build<T>(self) -> Result<ResponseListSuccessDto<Vec<T>>>
	where
		T: DeserializeOwned + Serialize,
	{
		let page = self.meta.page.unwrap_or(1).max(1);
		let per_page = self.meta.per_page.unwrap_or(10).max(1);
		let start = (page - 1) * per_page;

		let sql = crate::ListQueryBuilder::from_meta(
			self.table,
			self.meta,
			&self.search_field,
			self.select_fields,
			self.fetch_fields,
		)
		.build();

		let mut query_exec = self.db.query(sql);
		if let Some(search) = &self.meta.search {
			if !search.is_empty() {
				query_exec = query_exec.bind(("search", search.to_lowercase()));
			}
		}
		if let Some(filter_val) = &self.meta.filter {
			query_exec = crate::bind_filter_value(query_exec, filter_val.clone());
		}
		query_exec = query_exec
			.bind(("per_page", per_page))
			.bind(("start", start));

		let raw: Vec<T> = query_exec.await?.take(0)?;

		let mut count_query = self.db.query(format!(
			"SELECT count() FROM {} {}",
			self.table,
			if self.conditions.is_empty() {
				"".into()
			} else {
				format!("WHERE {}", self.conditions.join(" AND "))
			}
		));

		if let Some(search) = &self.meta.search {
			if !search.is_empty() {
				count_query = count_query.bind(("search", search.clone()));
			}
		}
		if let Some(filter_val) = &self.meta.filter {
			count_query = crate::bind_filter_value(count_query, filter_val.clone());
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
}
