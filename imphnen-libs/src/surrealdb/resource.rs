use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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
	Teams,
	TeamMembers,
	TeamInvitations,
}

impl fmt::Display for ResourceEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let str = match self {
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
		write!(f, "{str}")
	}
}
