use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RolesEnum {
	Admin,
	User,
	Staff,
}

impl fmt::Display for RolesEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let roles_str = match self {
			RolesEnum::Admin => "Admin",
			RolesEnum::User => "User",
			RolesEnum::Staff => "Staff",
		};
		write!(f, "{}", roles_str)
	}
}
