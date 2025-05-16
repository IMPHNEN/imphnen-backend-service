use imphnen_libs::MetaRequestDto;

pub struct ListQueryBuilder {
	resource: String,
	conditions: Vec<String>,
	limit: usize,
	start: usize,
	order_by: Option<String>,
	order: Option<String>,
	fetch: Vec<String>,
	select_fields: Vec<String>,
}

impl ListQueryBuilder {
	pub fn from_meta(
		resource: impl Into<String>,
		meta: &MetaRequestDto,
		search_field: impl Into<String>,
		select_fields: Option<Vec<&str>>,
	) -> Self {
		let mut builder = Self::new(resource)
			.with_search(meta.search.as_deref(), &search_field.into())
			.with_filter(meta.filter_by.as_deref(), meta.filter.as_deref())
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.with_pagination(meta.page, meta.per_page);

		if let Some(fields) = select_fields {
			builder = builder.with_select_fields(fields);
		}
		builder
	}

	pub fn new(resource: impl Into<String>) -> Self {
		Self {
			resource: resource.into(),
			conditions: vec!["is_deleted = false".into()],
			limit: 10,
			start: 0,
			order_by: None,
			order: None,
			fetch: vec![],
			select_fields: vec![],
		}
	}

	pub fn with_select_fields(mut self, fields: Vec<&str>) -> Self {
		self.select_fields = fields.into_iter().map(String::from).collect();
		self
	}

	pub fn with_search(mut self, search: Option<&str>, field: &str) -> Self {
		if let Some(search) = search {
			if !search.is_empty() {
				self
					.conditions
					.push(format!("string::contains({} ?? '', $search)", field));
			}
		}
		self
	}

	pub fn with_filter(mut self, field: Option<&str>, value: Option<&str>) -> Self {
		if let (Some(f), Some(v)) = (field, value) {
			if !v.is_empty() {
				self.conditions.push(format!("{} = $filter", f));
			}
		}
		self
	}

	pub fn with_pagination(
		mut self,
		page: Option<u64>,
		per_page: Option<u64>,
	) -> Self {
		let limit = per_page.unwrap_or(10).max(1);
		let page = page.unwrap_or(1).max(1);
		self.limit = limit as usize;
		self.start = ((page - 1) * limit) as usize;
		self
	}

	pub fn with_sorting(mut self, sort_by: Option<&str>, order: Option<&str>) -> Self {
		self.order_by = sort_by.map(|s| s.to_string());
		self.order = order.map(|o| o.to_uppercase());
		self
	}

	pub fn with_fetch(mut self, fetch: impl Into<String>) -> Self {
		self.fetch.push(fetch.into());
		self
	}

	pub fn build(self) -> String {
		let where_clause = if !self.conditions.is_empty() {
			format!("WHERE {}", self.conditions.join(" AND "))
		} else {
			String::new()
		};

		let order_clause = if let Some(field) = self.order_by {
			let ord = self.order.unwrap_or_else(|| "ASC".into());
			format!("ORDER BY {} {}", field, ord)
		} else {
			String::new()
		};

		let fetch_clause = if !self.fetch.is_empty() {
			format!("FETCH {}", self.fetch.join(", "))
		} else {
			String::new()
		};

		let select_clause = if self.select_fields.is_empty() {
			"*".to_string()
		} else {
			self.select_fields.join(", ")
		};

		format!(
			r#"
	      SELECT {} FROM {}
	      {}
	      {}
	      LIMIT {} START {}
	      {}
	   "#,
			select_clause,
			self.resource,
			where_clause,
			order_clause,
			self.limit,
			self.start,
			fetch_clause
		)
	}
}
