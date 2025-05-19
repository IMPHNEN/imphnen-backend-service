use imphnen_libs::MetaRequestDto;
use surrealdb::engine::remote::ws::Client;
use surrealdb::method::Query;
use surrealdb::sql::Thing;

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
		fetch_fields: Option<Vec<&str>>,
	) -> Self {
		let mut builder = Self::new(resource)
			.with_search(meta.search.as_deref(), &search_field.into())
			.with_filter(meta.filter_by.as_deref(), meta.filter.as_deref())
			.with_sorting(meta.sort_by.as_deref(), meta.order.as_deref())
			.with_fetch(fetch_fields)
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
				self.conditions.push(format!(
					"string::contains(string::lowercase({} ?? ''), string::lowercase($search))",
					field
				));
			}
		}
		self
	}

	pub fn with_filter(mut self, field: Option<&str>, value: Option<&str>) -> Self {
		if let (Some(f), Some(v)) = (field, value) {
			if !v.is_empty() {
				self.conditions.push(format!(
					"string::contains(string::join('', [{}]), $filter)",
					f
				));
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

	pub fn with_fetch(mut self, fetches: Option<Vec<&str>>) -> Self {
		if let Some(items) = fetches {
			self.fetch.extend(items.into_iter().map(String::from));
		}
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

pub struct DetailQueryBuilder {
	resource: String,
	id: Option<String>,
	thing: Option<String>,
	where_field: Option<String>,
	where_value: Option<String>,
	select_fields: Vec<String>,
	fetch_fields: Vec<String>,
}

impl DetailQueryBuilder {
	pub fn new(resource: impl Into<String>) -> Self {
		Self {
			resource: resource.into(),
			id: None,
			thing: None,
			where_field: None,
			where_value: None,
			select_fields: vec![],
			fetch_fields: vec![],
		}
	}

	pub fn with_id(mut self, id: impl Into<String>) -> Self {
		if self.where_field.is_some() || self.thing.is_some() {
			panic!("Cannot use with_id() after with_where() or with_thing()");
		}
		self.id = Some(id.into());
		self
	}

	pub fn with_thing(mut self, thing: &Thing) -> Self {
		if self.id.is_some() || self.where_field.is_some() {
			panic!("Cannot use with_thing() after with_id() or with_where()");
		}
		self.thing = Some(thing.to_string()); // app_users:uuid
		self.resource = thing.tb.clone(); // update resource dari thing
		self
	}

	pub fn with_where(mut self, field: impl Into<String>) -> Self {
		if self.id.is_some() || self.thing.is_some() {
			panic!("Cannot use with_where() after with_id() or with_thing()");
		}
		self.where_field = Some(field.into());
		self
	}

	pub fn where_value(mut self, value: impl Into<String>) -> Self {
		self.where_value = Some(value.into());
		self
	}

	pub fn with_select_fields(mut self, fields: Vec<&str>) -> Self {
		self.select_fields = fields.into_iter().map(String::from).collect();
		self
	}

	pub fn with_fetch(mut self, field: impl Into<String>) -> Self {
		self.fetch_fields.push(field.into());
		self
	}

	pub fn build(&self) -> String {
		let select_clause = if self.select_fields.is_empty() {
			"*".to_string()
		} else {
			self.select_fields.join(", ")
		};

		let fetch_clause = if self.fetch_fields.is_empty() {
			String::new()
		} else {
			format!("FETCH {}", self.fetch_fields.join(", "))
		};

		let from_clause = if let Some(thing) = &self.thing {
			thing.to_string()
		} else if let Some(id) = &self.id {
			format!("{}:⟨{}⟩", self.resource, id)
		} else if let (Some(field), Some(_)) = (&self.where_field, &self.where_value) {
			format!("{} WHERE {} = $value", self.resource, field)
		} else {
			panic!(
				"You must set one of with_id(), with_thing(), or with_where()+where_value()"
			);
		};

		format!(
			"SELECT {} FROM {} {}",
			select_clause, from_clause, fetch_clause
		)
	}

	pub fn apply_bindings<'q>(&self, query: Query<'q, Client>) -> Query<'q, Client> {
		if let (Some(_), Some(value)) = (&self.where_field, &self.where_value) {
			query.bind(("value", value.clone()))
		} else {
			query
		}
	}
}
