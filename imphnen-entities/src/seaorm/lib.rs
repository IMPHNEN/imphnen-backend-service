//! SeaORM entity definitions for Imphenia backend
//! Provides PostgreSQL-compatible entity definitions corresponding to SurrealDB ResourceEnum

pub mod auth;
pub mod gacha;
pub mod common;
pub mod relationships;
pub mod schema_validation;
pub mod examples;

// Re-export specific items from modules for better API clarity
pub use auth::{
    users, mentors, roles, permissions, roles_permissions, sessions
};
pub use gacha::{
    gacha_items, gacha_claims, gacha_credits, gacha_rolls
};
pub use common::{
    ResourceEnum, PgUuid, generate_uuid, current_timestamp,
    audit_log, rate_limit, events, testimonials
};
pub use relationships;
pub use schema_validation;
pub use examples;

/// Initialize the SeaORM entity system
/// Should be called once at application startup
pub fn initialize() -> Result<(), String> {
    // Perform schema validation on initialization
    validate_schema_equivalence()?;
    
    // Initialize any global utilities or configurations
    common::utils::initialize_utils();
    
    Ok(())
}

/// Get the table name for a given ResourceEnum
/// Provides a consistent way to access table names across the application
pub fn get_table_name(resource: &common::enums::ResourceEnum) -> &str {
    resource.as_str()
}

/// Get the schema name for all entities (default: "public")
pub fn get_schema_name() -> &str {
    "public"
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::enums::ResourceEnum;

    #[test]
    fn test_table_name_resolution() {
        assert_eq!(get_table_name(&ResourceEnum::Users), "app_users");
        assert_eq!(get_table_name(&ResourceEnum::Roles), "app_roles");
        assert_eq!(get_table_name(&ResourceEnum::GachaItems), "app_gacha_items");
    }

    #[test]
    fn test_schema_name() {
        assert_eq!(get_schema_name(), "public");
    }

    #[test]
    fn test_initialize() {
        // This should not panic and should return Ok(())
        let result = initialize();
        assert!(result.is_ok());
    }
}