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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roles_enum_display() {
        assert_eq!(format!("{}", RolesEnum::Admin), "Admin");
        assert_eq!(format!("{}", RolesEnum::Administrator), "Administrator");
        assert_eq!(format!("{}", RolesEnum::User), "User");
        assert_eq!(format!("{}", RolesEnum::Staff), "Staff");
        assert_eq!(format!("{}", RolesEnum::Mentor), "Mentor");
    }
}
