use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceEnum {
	GachaItems,
	GachaClaims,
	GachaRolls,
	Users,
	Roles,
	Permissions,
	RolesPermissions,
}

impl fmt::Display for ResourceEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
			ResourceEnum::Users => "app_users",
			ResourceEnum::Roles => "app_roles",
			ResourceEnum::Permissions => "app_permissions",
			ResourceEnum::RolesPermissions => "app_roles_permissions",
			ResourceEnum::GachaItems => "app_gacha_items",
			ResourceEnum::GachaClaims => "app_gacha_claims",
			ResourceEnum::GachaRolls => "app_gacha_rolls",
		};
		write!(f, "{}", str)
	}
}
