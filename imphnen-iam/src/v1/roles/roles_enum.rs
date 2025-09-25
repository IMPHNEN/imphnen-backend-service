use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolesEnum {
	Admin,
	Administrator, // Added Administrator role
	User,
	Staff,
	Mentor, // Added Mentor role
}

impl fmt::Display for RolesEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let roles_str = match self {
			RolesEnum::Admin => "Admin",
			RolesEnum::Administrator => "Administrator", // Added Administrator role
			RolesEnum::User => "User",
			RolesEnum::Staff => "Staff",
			RolesEnum::Mentor => "Mentor", // Added Mentor role
		};
		write!(f, "{roles_str}")
	}
}
