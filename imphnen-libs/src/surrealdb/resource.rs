//! SurrealDB resource definitions.
//!
//! This module defines the database table names used throughout the application.
//! Each resource corresponds to a SurrealDB table with the "app_" prefix.

use std::fmt;

/// Database resource enumeration.
///
/// Represents all database tables used in the application.
/// Each variant corresponds to a SurrealDB table name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Teams table for user teams
    Teams,
    /// Team members table for team membership
    TeamMembers,
    /// Team invitations table for pending invitations
    TeamInvitations,
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
            ResourceEnum::Teams => "app_teams",
            ResourceEnum::TeamMembers => "app_team_members",
            ResourceEnum::TeamInvitations => "app_team_invitations",
        };
        write!(f, "{}", table_name)
    }
}

impl ResourceEnum {
    /// Get the table name as a string slice.
    ///
    /// # Returns
    /// The SurrealDB table name for this resource
    ///
    /// # Example
    /// ```
    /// use imphnen_libs::ResourceEnum;
    ///
    /// let users = ResourceEnum::Users;
    /// assert_eq!(users.as_str(), "app_users");
    /// ```
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
            ResourceEnum::Teams => "app_teams",
            ResourceEnum::TeamMembers => "app_team_members",
            ResourceEnum::TeamInvitations => "app_team_invitations",
        }
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
}
