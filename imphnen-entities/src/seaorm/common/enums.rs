use super::types::PgUuid;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceEnum {
	OtpCache,
	UsersCache,
	GachaItems,
	GachaClaims,
	GachaRolls,
	GachaCredits,
	Users,
	Roles,
	Permissions,
	RolesPermissions,
	Events,
	Testimonials,
	Mentors,
	Notifications,
	RateLimit,
	AuditLog,
	Sessions,
	MigrationStatus,
}

impl ResourceEnum {
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

	pub fn schema(&self) -> &'static str {
		"public"
	}

	pub fn to_entity_name(&self) -> String {
		self.as_str().replace("app_", "").to_pascal_case()
	}

	pub fn is_cache(&self) -> bool {
		matches!(self, ResourceEnum::OtpCache | ResourceEnum::UsersCache)
	}

	pub fn is_gacha(&self) -> bool {
		matches!(
			self,
			ResourceEnum::GachaItems
				| ResourceEnum::GachaClaims
				| ResourceEnum::GachaRolls
				| ResourceEnum::GachaCredits
		)
	}

	pub fn is_user_related(&self) -> bool {
		matches!(
			self,
			ResourceEnum::Users | ResourceEnum::UsersCache | ResourceEnum::Mentors
		)
	}

	pub fn generate_ref_id(&self, uuid: &PgUuid) -> String {
		format!("{}_{}", self.as_str().replace("app_", ""), uuid.0)
	}
}

trait ToPascalCase {
	fn to_pascal_case(&self) -> String;
}

impl ToPascalCase for str {
	fn to_pascal_case(&self) -> String {
		self
			.split('_')
			.map(|s| {
				let mut chars = s.chars();
				chars
					.next()
					.map(|c| c.to_uppercase().collect::<String>() + chars.as_str())
					.unwrap_or_default()
			})
			.collect()
	}
}
