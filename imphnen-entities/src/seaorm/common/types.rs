//! Shared type definitions for SeaORM entities
//! Provides PostgreSQL-compatible type aliases and custom types

use chrono::{DateTime, Utc};

use uuid::Uuid;

/// UUID type alias for PostgreSQL UUID compatibility
/// Uses `Uuid` from the `uuid` crate with SeaORM conversion traits
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PgUuid(pub Uuid);

impl From<Uuid> for PgUuid {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<PgUuid> for Uuid {
    fn from(pg_uuid: PgUuid) -> Self {
        pg_uuid.0
    }
}

impl From<PgUuid> for String {
    fn from(pg_uuid: PgUuid) -> Self {
        pg_uuid.0.to_string()
    }
}

/// Timestamp type alias for PostgreSQL TIMESTAMP with time zone
/// Uses `DateTime<Utc>` from the `chrono` crate
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PgTimestamp(pub DateTime<Utc>);

impl From<DateTime<Utc>> for PgTimestamp {
    fn from(timestamp: DateTime<Utc>) -> Self {
        Self(timestamp)
    }
}

impl From<PgTimestamp> for DateTime<Utc> {
    fn from(pg_timestamp: PgTimestamp) -> Self {
        pg_timestamp.0
    }
}

/// JSONB type alias for PostgreSQL JSONB compatibility
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PgJsonB<T>(pub T);

impl<T> From<T> for PgJsonB<T>
where
    T: serde::Serialize,
{
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// Common fields that should be included in all entities
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CommonFields {
    pub id: PgUuid,
    pub created_at: PgTimestamp,
    pub updated_at: PgTimestamp,
    pub deleted_at: Option<PgTimestamp>,
}

// Helper macros for common field definitions
#[macro_export]
macro_rules! common_fields {
    () => {
        pub id: ColumnDef<Uuid> = ColumnDef::new(sea_orm::sea_query::Column::new("id"))
            .primary_key()
            .not_null()
            .default(sea_orm::sea_query::Expr::cust("gen_random_uuid()")),
        pub created_at: ColumnDef<DateTime<Utc>> = ColumnDef::new(sea_orm::sea_query::Column::new("created_at"))
            .not_null()
            .default(sea_orm::sea_query::Expr::cust("now()")),
        pub updated_at: ColumnDef<DateTime<Utc>> = ColumnDef::new(sea_orm::sea_query::Column::new("updated_at"))
            .not_null()
            .default(sea_orm::sea_query::Expr::cust("now()"))
            .extra(sea_orm::sea_query::PostgresExtension::new("GENERATED ALWAYS AS (now()) STORED")),
        pub deleted_at: ColumnDef<Option<DateTime<Utc>>> = ColumnDef::new(sea_orm::sea_query::Column::new("deleted_at"))
            .default(None),
    };
}

