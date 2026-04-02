use serde::{Deserialize, Serialize};
use std::fmt;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, strum::EnumIter)]
pub enum PermissionsEnum {
	ReadListUsers,
	ReadDetailUsers,
	CreateUsers,
	DeleteUsers,
	UpdateUsers,
	ActivateUsers,

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

	ManageAllUsers,
	ManageAllRoles,
	ManageAllPermissions,
	ViewAllSensitiveData,
	AccessAdminDashboard,
	Administrator,

	CreateGachaClaims,
	ReadDetailGachaClaims,
	ReadListGachaItems,
	ReadDetailGachaItems,
	CreateGachaItems,
	DeleteGachaItems,
	UpdateGachaItems,
	ReadDetailGachaRolls,
	CreateGachaRolls,
	ExecuteGachaRolls,
	DeleteGachaRolls,

	ReadListMentors,
	ReadDetailMentors,
	RegisterMentors,
	ReadOwnMentorProfile,
	UpdateOwnMentorProfile,
	ReadOwnMentorStatus,
	UpdateMentors,
	VerifyMentors,
	DeleteMentors,
}

impl fmt::Display for PermissionsEnum {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let permission_str = match self {
			PermissionsEnum::ReadListUsers => "Read List Users",
			PermissionsEnum::ReadDetailUsers => "Read Detail Users",
			PermissionsEnum::CreateUsers => "Create Users",
			PermissionsEnum::DeleteUsers => "Delete Users",
			PermissionsEnum::UpdateUsers => "Update Users",
			PermissionsEnum::ActivateUsers => "Activate Users",

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

			PermissionsEnum::CreateGachaClaims => "Create Gacha Claims",
			PermissionsEnum::ReadDetailGachaClaims => "Read Detail Gacha Claims",
			PermissionsEnum::ReadListGachaItems => "Read List Gacha Items",
			PermissionsEnum::ReadDetailGachaItems => "Read Detail Gacha Items",
			PermissionsEnum::CreateGachaItems => "Create Gacha Items",
			PermissionsEnum::DeleteGachaItems => "Delete Gacha Items",
			PermissionsEnum::UpdateGachaItems => "Update Gacha Items",
			PermissionsEnum::ReadDetailGachaRolls => "Read Detail Gacha Rolls",
			PermissionsEnum::CreateGachaRolls => "Create Gacha Rolls",
			PermissionsEnum::ExecuteGachaRolls => "Execute Gacha Rolls",
			PermissionsEnum::DeleteGachaRolls => "Delete Gacha Rolls",

			PermissionsEnum::ReadListMentors => "Read List Mentors",
			PermissionsEnum::ReadDetailMentors => "Read Detail Mentors",
			PermissionsEnum::RegisterMentors => "Register Mentors",
			PermissionsEnum::ReadOwnMentorProfile => "Read Own Mentor Profile",
			PermissionsEnum::UpdateOwnMentorProfile => "Update Own Mentor Profile",
			PermissionsEnum::ReadOwnMentorStatus => "Read Own Mentor Status",
			PermissionsEnum::UpdateMentors => "Update Mentors",
			PermissionsEnum::VerifyMentors => "Verify Mentors",
			PermissionsEnum::DeleteMentors => "Delete Mentors",

			PermissionsEnum::ManageAllUsers => "Manage All Users",
			PermissionsEnum::ManageAllRoles => "Manage All Roles",
			PermissionsEnum::ManageAllPermissions => "Manage All Permissions",
			PermissionsEnum::ViewAllSensitiveData => "View All Sensitive Data",
			PermissionsEnum::AccessAdminDashboard => "Access Admin Dashboard",
			PermissionsEnum::Administrator => "Administrator",
		};
		write!(f, "{permission_str}")
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PermissionsItemDto {
	pub id: String,
	pub name: String,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl PermissionsItemDto {
	pub fn from(dto: &PermissionsQueryDto) -> Self {
		Self {
			id: dto.id.clone().unwrap_or_default(),
			name: dto.name.clone().unwrap_or_default(),
			created_at: dto.created_at.clone(),
			updated_at: dto.updated_at.clone(),
		}
	}
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PermissionsQueryDto {
	pub id: Option<String>,
	pub name: Option<String>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
}

impl PermissionsEnum {
	pub fn generate_id() -> String {
		Uuid::new_v4().to_string()
	}
}
