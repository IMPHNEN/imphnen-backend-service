//! Query builder utilities for SurrealDB.
//!
//! This module provides builders for constructing SurrealDB queries with
//! support for pagination, filtering, sorting, and binding parameters.
//! Includes both list queries and detail queries with unique binding keys.

use anyhow::Result;
use imphnen_libs::MetaRequestDto;
use serde_json::{Map, Value};
use surrealdb::engine::any;
use surrealdb::method::Query;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

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
					"string::contains(string::lowercase({field} ?? ''), string::lowercase($search))"
				));
			}
		}
		self
	}

	pub fn with_filter(mut self, field: Option<&str>, value: Option<&str>) -> Self {
		if let (Some(f), Some(v)) = (field, value) {
			if !v.is_empty() {
				self.conditions.push(format!(
					"string::contains(string::join('', [{f}]), $filter)"
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
			format!("ORDER BY {field} {ord}")
		} else {
			String::new()
		};

		let fetch_clause = if !self.fetch.is_empty() {
			format!("FETCH {}", self.fetch.join(", "))
		} else {
			String::new()
		};

		let select_clause = if self.select_fields.is_empty() {
			"*"
		} else {
			&self.select_fields.join(", ")
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

	pub fn build_count(self) -> String {
		let where_clause = if !self.conditions.is_empty() {
			format!("WHERE {}", self.conditions.join(" AND "))
		} else {
			String::new()
		};

		format!("SELECT count() FROM {} {}", self.resource, where_clause)
	}
}

pub struct DetailQueryBuilder {
	resource: String,
	id: Option<String>,
	thing: Option<String>,
	select_fields: Vec<String>,
	fetch_fields: Vec<String>,
	conditions: Vec<String>,
	bindings: Map<String, Value>,
	binding_counter: usize,
}

impl DetailQueryBuilder {
	pub fn new(resource: impl Into<String>) -> Self {
		Self {
			resource: resource.into(),
			id: None,
			thing: None,
			select_fields: vec![],
			fetch_fields: vec![],
			conditions: vec![],
			bindings: Map::new(),
			binding_counter: 0,
		}
	}

	pub fn with_id(mut self, id: impl Into<String>) -> Self {
		if self.thing.is_some() || !self.conditions.is_empty() {
			panic!(
				"Cannot use with_id() after with_thing() or with_where()/with_condition()"
			);
		}
		self.id = Some(id.into());
		self
	}

	pub fn with_thing(mut self, thing: &Thing) -> Self {
		if self.id.is_some() || !self.conditions.is_empty() {
			panic!(
				"Cannot use with_thing() after with_id() or with_where()/with_condition()"
			);
		}
		self.thing = Some(thing.to_string());
		self.resource = thing.tb.to_string();
		self
	}

	pub fn with_where(
		mut self,
		field: impl Into<String>,
		value: Option<impl Into<String>>,
	) -> Self {
		if self.id.is_some() || self.thing.is_some() {
			panic!("Cannot use with_where() after with_id() or with_thing()");
		}
		let field_str = field.into();
		if let Some(val) = value {
			// Using a unique binding key to avoid conflicts
			let key = format!("value_where_{}", self.binding_counter);
			self.binding_counter += 1;
			self.conditions.push(format!("{field_str} = ${key}"));
			self.bindings.insert(key, Value::String(val.into()));
		} else {
			// If no value, assume it's a direct condition string (e.g., "is_active = true")
			self.conditions.push(field_str);
		}
		self
	}

	pub fn with_condition(mut self, condition: &str) -> Self {
		self.conditions.push(condition.to_string());
		self
	}

	pub fn with_thing_equals(mut self, field: &str, thing: &Thing) -> Self {
		let condition = build_thing_condition(field, thing);
		self.conditions.push(condition);
		self
	}

	pub fn with_things_equals(mut self, conditions: &[(&str, &Thing)]) -> Self {
		let condition = build_multi_thing_condition(conditions);
		self.conditions.push(condition);
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
			"*"
		} else {
			&self.select_fields.join(", ")
		};

		let fetch_clause = if self.fetch_fields.is_empty() {
			""
		} else {
			&format!("FETCH {}", self.fetch_fields.join(", "))
		};

		// Determine the base FROM clause
		let from_clause_base = if let Some(thing) = &self.thing {
			thing.as_str()
		} else if let Some(id_val) = &self.id {
			&format!("{}:⟨{}⟩", self.resource, id_val)
		} else {
			&self.resource
		};

		// Add WHERE clause based on accumulated conditions
		let final_from_clause = if !self.conditions.is_empty() {
			format!("{} WHERE {}", from_clause_base, self.conditions.join(" AND "))
		} else {
			from_clause_base.to_string()
		};

		format!("SELECT {select_clause} FROM {final_from_clause} {fetch_clause}")
	}

	pub fn apply_bindings<'q>(
		&self,
		mut query: Query<'q, any::Any>,
	) -> Query<'q, any::Any> {
		for (key, val) in &self.bindings {
			query = query.bind((key.clone(), val.clone()));
		}
		query
	}
}

pub fn build_thing_condition(field: &str, thing: &Thing) -> String {
	format!("{} = type::thing('{}', '{}')", field, thing.tb, thing.id.to_raw())
}

pub fn build_multi_thing_condition(conditions: &[(&str, &Thing)]) -> String {
	conditions
		.iter()
		.map(|(field, thing)| build_thing_condition(field, thing))
		.collect::<Vec<_>>()
		.join(" AND ")
}

pub async fn execute_safe_update_query(
	db: &Surreal<surrealdb::engine::any::Any>,
	query: String,
) -> Result<()> {
	let mut result = db.query(query).await?;
	let _: Result<Vec<serde_json::Value>, _> = result.take(0);
	Ok(())
}

pub async fn execute_safe_count_query(
	db: &Surreal<surrealdb::engine::any::Any>,
	resource: String,
	conditions: &str,
) -> Result<u64> {
	let query = format!("SELECT count() FROM {} WHERE {}", resource, conditions);
	let mut result = db.query(query).await?;
	
	// Extract the count from the result
	let response: Vec<surrealdb::Value> = result.take(0)?;
	let count = response.get(0).and_then(|v| v.to_string().parse::<u64>().ok())
		.ok_or_else(|| anyhow::anyhow!("No count found in response"))?;
	
	Ok(count)
}

#[cfg(test)]
mod query_builder_tests {
	use super::*;
	use crate::make_thing_from_enum;
	use imphnen_libs::ResourceEnum;

	#[test]
	fn test_build_thing_condition() {
		let team_thing = make_thing_from_enum(ResourceEnum::Teams, "test-id");
		let condition = build_thing_condition("team_id", &team_thing);
		assert_eq!(condition, "team_id = type::thing('app_teams', 'test-id')");
	}

	#[test]
	fn test_build_multi_thing_condition() {
		let team_thing = make_thing_from_enum(ResourceEnum::Teams, "team-id");
		let user_thing = make_thing_from_enum(ResourceEnum::Users, "user-id");
		
		let conditions = build_multi_thing_condition(&[
			("team_id", &team_thing),
			("user_id", &user_thing),
		]);
		
		assert_eq!(
			conditions,
			"team_id = type::thing('app_teams', 'team-id') AND user_id = type::thing('app_users', 'user-id')"
		);
	}
}
