//! Enum definitions for SeaORM entities
//! Provides resource type enumerations matching SurrealDB ResourceEnum

use std::fmt;
use serde::{Deserialize, Serialize};

use super::types::PgUuid;

/// Database resource enumeration for SeaORM
/// Matches the SurrealDB ResourceEnum with PostgreSQL compatibility
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceEnum {
    /// OTP cache table for temporary authentication codes
    OtpCache,
    /// User cache table for user session data
    UsersCache,
    /// Gacha items table
    GachaItems,
    /// Gacha claims table for user item claims
    GachaClaims,
    /// Gacha rolls table for user roll history
    GachaRolls,
    /// Gacha credits table for user currency
    GachaCredits,
    /// Users table for user accounts
    Users,
    /// Roles table for user roles
    Roles,
    /// Permissions table for system permissions
    Permissions,
    /// Role-permission relationships table
    RolesPermissions,
    /// Events table for application events
    Events,
    /// Testimonials table for user testimonials
    Testimonials,
    /// Mentors table for mentor profiles
    Mentors,
    /// Notifications table for user notifications
    Notifications,
    /// Rate limiting table for IP-based rate limiting
    RateLimit,
    /// Audit log table for admin action tracking
        AuditLog,
        /// Sessions table for mentoring sessions
        Sessions,
        /// Migration status tracking table
        MigrationStatus,
}

impl fmt::Display for ResourceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let table_name = match self {
            ResourceEnum::Users => "app_users",
            ResourceEnum::UsersCache => "app_users_cache",
            ResourceEnum::OtpCache => "app_otp_cache",
            ResourceEnum::Roles => "app_roles",
            ResourceEnum::Permissions => "app_permissions",
            ResourceEnum::RolesPermissions => "app_roles_permissions",
            ResourceEnum::GachaItems => "app_gacha_items",
            ResourceEnum::GachaClaims => "app_gacha_claims",
            ResourceEnum::GachaRolls => "app_gacha_rolls",
            ResourceEnum::GachaCredits => "app_gacha_credits",
            ResourceEnum::Events => "app_events",
            ResourceEnum::Testimonials => "app_testimonials",
            ResourceEnum::Mentors => "app_mentors",
            ResourceEnum::Notifications => "app_notifications",
            ResourceEnum::RateLimit => "app_rate_limit",
            ResourceEnum::AuditLog => "app_audit_log",
            ResourceEnum::Sessions => "app_sessions",
                        ResourceEnum::MigrationStatus => "app_migration_status",
        };
        write!(f, "{}", table_name)
    }
}

impl ResourceEnum {
    /// Get the table name as a string slice.
    ///
    /// # Returns
    /// The PostgreSQL table name for this resource
    pub fn as_str(&self) -> &'static str {
        match self {
            ResourceEnum::Users => "app_users",
            ResourceEnum::UsersCache => "app_users_cache",
            ResourceEnum::OtpCache => "app_otp_cache",
            ResourceEnum::Roles => "app_roles",
            ResourceEnum::Permissions => "app_permissions",
            ResourceEnum::RolesPermissions => "app_roles_permissions",
            ResourceEnum::GachaItems => "app_gacha_items",
            ResourceEnum::GachaClaims => "app_gacha_claims",
            ResourceEnum::GachaRolls => "app_gacha_rolls",
            ResourceEnum::GachaCredits => "app_gacha_credits",
            ResourceEnum::Events => "app_events",
            ResourceEnum::Testimonials => "app_testimonials",
            ResourceEnum::Mentors => "app_mentors",
            ResourceEnum::Notifications => "app_notifications",
            ResourceEnum::RateLimit => "app_rate_limit",
            ResourceEnum::AuditLog => "app_audit_log",
            ResourceEnum::Sessions => "app_sessions",
                        ResourceEnum::MigrationStatus => "app_migration_status",
        }
    }

    /// Get the schema name for the resource
    ///
    /// # Returns
    /// The database schema name (usually "public" for PostgreSQL)
    pub fn schema(&self) -> &'static str {
        "public"
    }

    /// Create a SeaORM entity name from the resource enum
    ///
    /// # Returns
    /// A string suitable for use as a SeaORM entity name
    pub fn to_entity_name(&self) -> String {
        self.as_str().replace("app_", "").to_pascal_case()
    }

    /// Check if this resource is cache-related.
    ///
    /// # Returns
    /// true if the resource is used for caching, false otherwise
    pub fn is_cache(&self) -> bool {
        matches!(self, ResourceEnum::OtpCache | ResourceEnum::UsersCache)
    }

    /// Check if this resource is gacha-related.
    ///
    /// # Returns
    /// true if the resource is part of the gacha system, false otherwise
    pub fn is_gacha(&self) -> bool {
        matches!(
            self,
            ResourceEnum::GachaItems
                | ResourceEnum::GachaClaims
                | ResourceEnum::GachaRolls
                | ResourceEnum::GachaCredits
        )
    }

    /// Check if this resource is user-related.
    ///
    /// # Returns
    /// true if the resource contains user data, false otherwise
    pub fn is_user_related(&self) -> bool {
        matches!(
            self,
            ResourceEnum::Users | ResourceEnum::UsersCache | ResourceEnum::Mentors
        )
    }

    /// Generate a reference ID for the resource
    ///
    /// # Returns
    /// A formatted string suitable for use as a reference ID
    pub fn generate_ref_id(&self, uuid: &PgUuid) -> String {
        format!("{}_{}", self.as_str().replace("app_", ""), uuid.0)
    }
}

// Helper trait for string case conversion
trait ToPascalCase {
    fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for str {
    fn to_pascal_case(&self) -> String {
        self.split('_')
            .map(|s| s.chars().next().unwrap().to_uppercase().to_string() + &s[1..])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_enum_table_names() {
        assert_eq!(ResourceEnum::Users.as_str(), "app_users");
        assert_eq!(ResourceEnum::Roles.as_str(), "app_roles");
        assert_eq!(ResourceEnum::GachaItems.as_str(), "app_gacha_items");
    }

    #[test]
    fn test_resource_enum_display() {
        assert_eq!(format!("{}", ResourceEnum::Users), "app_users");
        assert_eq!(format!("{}", ResourceEnum::RolesPermissions), "app_roles_permissions");
    }

    #[test]
    fn test_resource_enum_categories() {
        assert!(ResourceEnum::Users.is_user_related());
        assert!(ResourceEnum::GachaItems.is_gacha());
        assert!(ResourceEnum::OtpCache.is_cache());
    }

    #[test]
    fn test_resource_enum_to_entity_name() {
        assert_eq!(ResourceEnum::Users.to_entity_name(), "Users");
        assert_eq!(ResourceEnum::RolesPermissions.to_entity_name(), "RolesPermissions");
        assert_eq!(ResourceEnum::GachaItems.to_entity_name(), "GachaItems");
    }
}