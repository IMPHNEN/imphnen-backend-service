use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PermissionsEnum {
	ReadListUsers,
	ReadDetailUsers,
	CreateUsers,
	DeleteUsers,
	UpdateUsers,
	ReadListRoles,
	ReadDetailRoles,
	CreateRoles,
	DeleteRoles,
	UpdateRoles,
	ReadListPermissions,
	ReadDetailPermissions,
	CreatePermissions,
	DeletePermissions,
	UpdatePermissions,
}

impl fmt::Display for PermissionsEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let permission_str = match self {
			PermissionsEnum::ReadListUsers => "Read List Users",
			PermissionsEnum::ReadDetailUsers => "Read Detail Users",
			PermissionsEnum::CreateUsers => "Create Users",
			PermissionsEnum::DeleteUsers => "Delete Users",
			PermissionsEnum::UpdateUsers => "Update Users",
			PermissionsEnum::ReadListRoles => "Read List Roles",
			PermissionsEnum::ReadDetailRoles => "Read Detail Roles",
			PermissionsEnum::CreateRoles => "Create Roles",
			PermissionsEnum::DeleteRoles => "Delete Roles",
			PermissionsEnum::UpdateRoles => "Update Roles",
			PermissionsEnum::ReadListPermissions => "Read List Permissions",
			PermissionsEnum::ReadDetailPermissions => "Read Detail Permissions",
			PermissionsEnum::CreatePermissions => "Create Permissions",
			PermissionsEnum::DeletePermissions => "Delete Permissions",
			PermissionsEnum::UpdatePermissions => "Update Permissions",
		};
		write!(f, "{}", permission_str)
	}
}
