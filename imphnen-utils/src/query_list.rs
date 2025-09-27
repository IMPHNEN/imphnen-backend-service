use anyhow::Result;
use imphnen_entities::{
    CountResult, MetaRequestDto, MetaResponseDto, ResponseListSuccessDto,
};
use serde::{de::DeserializeOwned, Serialize};
use surrealdb::engine::any;
use surrealdb::Surreal;
use tracing;

pub struct QueryListBuilder<'a> {
    db: &'a Surreal<any::Any>,
    table: &'a str,
    meta: &'a MetaRequestDto,
    conditions: Vec<String>,
    search_field: String,
    select_fields: Option<Vec<&'a str>>,
    fetch_fields: Option<Vec<&'a str>>,
    cast_thing_fields: bool,
}

impl<'a> QueryListBuilder<'a> {
    pub fn new(
        db: &'a Surreal<any::Any>,
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
            cast_thing_fields: false,
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

    pub fn with_cast_thing_fields(mut self) -> Self {
        self.cast_thing_fields = true;
        self
    }

    pub async fn build<T>(self) -> Result<ResponseListSuccessDto<Vec<T>>>
    where
        T: DeserializeOwned + Serialize,
    {
        let page = self.meta.page.unwrap_or(1).max(1);
        let per_page = self.meta.per_page.unwrap_or(10).max(1);
        let start = (page - 1) * per_page;

        // --- Data Query ---
        let data_query_builder = crate::ListQueryBuilder::from_meta(
            self.table,
            self.meta,
            &self.search_field,
            self.select_fields,
            self.fetch_fields,
        ).with_additional_conditions(&self.conditions);
        let data_sql = data_query_builder.build();

        // --- Count Query ---
        let count_query_builder = crate::ListQueryBuilder::from_meta(
            self.table,
            self.meta,
            &self.search_field,
            None, // No select fields for count
            None, // No fetch fields for count
        ).with_additional_conditions(&self.conditions);
        let count_sql = count_query_builder.build_count();

        // Combine both queries into a single query string within a transaction for a single database call
        let combined_sql = format!(
            "BEGIN; {}; {}; COMMIT;",
            data_sql,
            count_sql
        );

        let mut query_exec = self.db.query(combined_sql.clone());

        // Bind parameters for both data and count queries.
        // It's assumed that the parameters are named consistently and applied to both.
        // The ListQueryBuilder already uses $search, $per_page, $start, $filter.
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

        let query_debug_str = format!("{:?}", &query_exec);

        let mut response = query_exec.await.map_err(|e| {
            tracing::error!(
                query = %combined_sql, // `combined_sql` is cloned, so it can be borrowed here
                full_query_object = %query_debug_str,
                "Failed to execute combined query: {:?}", e
            );
            e
        })?;

        // Extract results: first for the data, then for the count
        let raw: Vec<T> = response.take(0)?; // First result is the data
        let count_result: Vec<CountResult> = response.take(1)?; // Second result is the count

        let total = count_result.first().map(|c| c.count);
        // Debug logging
        if std::env::var("RUST_ENV").unwrap_or_else(|_| "development".to_string()) == "development" {
            println!("QueryListBuilder: data length = {}, total from count = {:?}", raw.len(), total);
            println!("Combined SQL: {}", combined_sql);
        }

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
