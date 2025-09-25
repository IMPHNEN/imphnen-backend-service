use std::fmt;
use uuid::Uuid;
use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
pub enum PermissionsEnum {
	// User permissions
	ReadListUsers,
	ReadDetailUsers,
	CreateUsers,
	DeleteUsers,
	UpdateUsers,
	ActivateUsers,
	
	// Role permissions
	ReadListRoles,
	ReadDetailRoles,
	CreateRoles,
	DeleteRoles,
	UpdateRoles,
	
	// Permission permissions
	ReadListPermissions,
	ReadDetailPermissions,
	CreatePermissions,
	DeletePermissions,
	UpdatePermissions,
	
	// Team permissions
	ReadListTeams,
	ReadDetailTeams,
	
	// Gacha permissions
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
	
	// Mentor permissions
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
			// User permissions
			PermissionsEnum::ReadListUsers => "Read List Users",
			PermissionsEnum::ReadDetailUsers => "Read Detail Users",
			PermissionsEnum::CreateUsers => "Create Users",
			PermissionsEnum::DeleteUsers => "Delete Users",
			PermissionsEnum::UpdateUsers => "Update Users",
			PermissionsEnum::ActivateUsers => "Activate Users",
			
			// Role permissions
			PermissionsEnum::ReadListRoles => "Read List Roles",
			PermissionsEnum::ReadDetailRoles => "Read Detail Roles",
			PermissionsEnum::CreateRoles => "Create Roles",
			PermissionsEnum::DeleteRoles => "Delete Roles",
			PermissionsEnum::UpdateRoles => "Update Roles",
			
			// Permission permissions
			PermissionsEnum::ReadListPermissions => "Read List Permissions",
			PermissionsEnum::ReadDetailPermissions => "Read Detail Permissions",
			PermissionsEnum::CreatePermissions => "Create Permissions",
			PermissionsEnum::DeletePermissions => "Delete Permissions",
			PermissionsEnum::UpdatePermissions => "Update Permissions",
			
			// Team permissions
			PermissionsEnum::ReadListTeams => "Read List Teams",
			PermissionsEnum::ReadDetailTeams => "Read Detail Teams",
			
			// Gacha permissions
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
			
			// Mentor permissions
			PermissionsEnum::ReadListMentors => "Read List Mentors",
			PermissionsEnum::ReadDetailMentors => "Read Detail Mentors",
			PermissionsEnum::RegisterMentors => "Register Mentors",
			PermissionsEnum::ReadOwnMentorProfile => "Read Own Mentor Profile",
			PermissionsEnum::UpdateOwnMentorProfile => "Update Own Mentor Profile",
			PermissionsEnum::ReadOwnMentorStatus => "Read Own Mentor Status",
			PermissionsEnum::UpdateMentors => "Update Mentors",
			PermissionsEnum::VerifyMentors => "Verify Mentors",
			PermissionsEnum::DeleteMentors => "Delete Mentors",
			
			// Administrator permissions
			PermissionsEnum::ManageAllUsers => "Manage All Users",
			PermissionsEnum::ManageAllRoles => "Manage All Roles",
			PermissionsEnum::ManageAllPermissions => "Manage All Permissions",
			PermissionsEnum::ManageAllTeams => "Manage All Teams",
			PermissionsEnum::ViewAllSensitiveData => "View All Sensitive Data",
			PermissionsEnum::AccessAdminDashboard => "Access Admin Dashboard",
		};
		write!(f, "{permission_str}")
	}
}

impl PermissionsEnum {
	pub fn id(&self) -> String {
		match self {
			// User permissions
			PermissionsEnum::ReadListUsers => "7c15e31d-36e2-49f9-97db-138c03fb0cf6".to_string(),
			PermissionsEnum::ReadDetailUsers => "319ee593-ff0a-4f29-bbaf-9feb3174a3a6".to_string(),
			PermissionsEnum::CreateUsers => "023e2dfe-93c3-4008-94a8-b5dff403f73b".to_string(),
			PermissionsEnum::DeleteUsers => "96df0689-2ae9-4894-bf00-837c19415e5c".to_string(),
			PermissionsEnum::UpdateUsers => "98b3dc4c-0124-461f-afcd-166637c5e6e8".to_string(),
			PermissionsEnum::ActivateUsers => "4da8b434-89f9-4d91-85ae-eebd63cdbeda".to_string(),
			
			// Role permissions
			PermissionsEnum::ReadListRoles => "9164ca6e-c7e3-4238-a15f-f36ab9577e7e".to_string(),
			PermissionsEnum::ReadDetailRoles => "73888d18-b3e9-4f62-95a5-ba2c0d69fccb".to_string(),
			PermissionsEnum::CreateRoles => "319ee593-ff0a-4f29-bbaf-9feb3174a3a2".to_string(),
			PermissionsEnum::DeleteRoles => "35b0d992-65c8-4b62-b030-e6e0320e4048".to_string(),
			PermissionsEnum::UpdateRoles => "a00d5608-4c48-4542-845c-dfe004687022".to_string(),
			
			// Permission permissions
			PermissionsEnum::ReadListPermissions => "8195eeb8-e64f-4172-aa57-596492c84a72".to_string(),
			PermissionsEnum::ReadDetailPermissions => "dad435cf-042c-41bd-a946-cea61ed2ffbc".to_string(),
			PermissionsEnum::CreatePermissions => "0269ed71-0ae0-4c43-ad29-e3d861d8f9a0".to_string(),
			PermissionsEnum::DeletePermissions => "b2dc3928-86ba-4c59-a03d-0b57d5183ebc".to_string(),
			PermissionsEnum::UpdatePermissions => "299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b".to_string(),
			
			// Team permissions
			PermissionsEnum::ReadListTeams => "e1f23456-7890-1234-5678-90abcdef1234".to_string(),
			PermissionsEnum::ReadDetailTeams => "f2345678-8901-2345-6789-01bcdef23456".to_string(),
			
			// Gacha permissions
			PermissionsEnum::CreateGachaClaims => "f41d53ce-4f88-4bb6-b9b4-5e3a8c38d962".to_string(),
			PermissionsEnum::ReadDetailGachaClaims => "c1c3d6c2-19fb-4b70-b58c-c19f2e8cfc79".to_string(),
			PermissionsEnum::ReadListGachaItems => "fa6eb842-0a61-40c2-9c24-b226ad975037".to_string(),
			PermissionsEnum::ReadDetailGachaItems => "9c7857d7-b5ae-4688-923d-ef5572e9bc8b".to_string(),
			PermissionsEnum::CreateGachaItems => "cf063be1-4d71-489e-b9fb-1c08c65f396c".to_string(),
			PermissionsEnum::DeleteGachaItems => "46f8c6cf-ea0c-4c90-860c-69e2e65f7eb1".to_string(),
			PermissionsEnum::UpdateGachaItems => "2d0cf4ae-56ae-4714-a12e-655cfc3d9eb2".to_string(),
			PermissionsEnum::ReadDetailGachaRolls => "53d6483a-04cd-4667-8792-2d0cc8e2d343".to_string(),
			PermissionsEnum::CreateGachaRolls => "18e36c63-fcb7-4877-b911-c5aa611e878f".to_string(),
			PermissionsEnum::ExecuteGachaRolls => "14c6a1cd-5c63-4643-89b5-b1a5f9920cc0".to_string(),
			PermissionsEnum::DeleteGachaRolls => "12345678-ABCD-EFAB-CDEF-0123456789AB".to_string(),
			
			// Mentor permissions
			PermissionsEnum::ReadListMentors => "a1b2c3d4-5e6f-7890-abcd-ef1234567890".to_string(),
			PermissionsEnum::ReadDetailMentors => "b2c3d4e5-6f78-9012-bcde-f23456789012".to_string(),
			PermissionsEnum::RegisterMentors => "c3d4e5f6-7890-1234-cdef-345678901234".to_string(),
			PermissionsEnum::ReadOwnMentorProfile => "d4e5f6a7-8901-2345-def0-456789012345".to_string(),
			PermissionsEnum::UpdateOwnMentorProfile => "e5f6a7b8-9012-3456-ef01-567890123456".to_string(),
			PermissionsEnum::ReadOwnMentorStatus => "f6a7b8c9-0123-4567-f012-678901234567".to_string(),
			PermissionsEnum::UpdateMentors => "a7b8c9d0-1234-5678-0123-789012345678".to_string(),
			PermissionsEnum::VerifyMentors => "b8c9d0e1-2345-6789-1234-890123456789".to_string(),
			PermissionsEnum::DeleteMentors => "c9d0e1f2-3456-7890-2345-901234567890".to_string(),
			
			// Administrator permissions
			PermissionsEnum::ManageAllUsers => "d0e1f2a3-4567-8901-2345-0123456789ab".to_string(),
			PermissionsEnum::ManageAllRoles => "e1f2a3b4-5678-9012-3456-1234567890ab".to_string(),
			PermissionsEnum::ManageAllPermissions => "f2a3b4c5-6789-0123-4567-2345678901ab".to_string(),
			PermissionsEnum::ManageAllTeams => "a3b4c5d6-7890-1234-5678-3456789012ab".to_string(),
			PermissionsEnum::ViewAllSensitiveData => "b4c5d6e7-8901-2345-6789-4567890123ab".to_string(),
			PermissionsEnum::AccessAdminDashboard => "c5d6e7f8-9012-3456-7890-5678901234ab".to_string(),
		}
	}
	
	/// Generate a new unique ID for a permission
	pub fn generate_id() -> String {
		Uuid::new_v4().to_string()
	}
	
	/// Get all permissions as a vector
	pub fn all() -> Vec<PermissionsEnum> {
		vec![
			// User permissions
			PermissionsEnum::ReadListUsers,
			PermissionsEnum::ReadDetailUsers,
			PermissionsEnum::CreateUsers,
			PermissionsEnum::DeleteUsers,
			PermissionsEnum::UpdateUsers,
			PermissionsEnum::ActivateUsers,
			
			// Role permissions
			PermissionsEnum::ReadListRoles,
			PermissionsEnum::ReadDetailRoles,
			PermissionsEnum::CreateRoles,
			PermissionsEnum::DeleteRoles,
			PermissionsEnum::UpdateRoles,
			
			// Permission permissions
			PermissionsEnum::ReadListPermissions,
			PermissionsEnum::ReadDetailPermissions,
			PermissionsEnum::CreatePermissions,
			PermissionsEnum::DeletePermissions,
			PermissionsEnum::UpdatePermissions,
			
			// Team permissions
			PermissionsEnum::ReadListTeams,
			PermissionsEnum::ReadDetailTeams,
			
			// Gacha permissions
			PermissionsEnum::CreateGachaClaims,
			PermissionsEnum::ReadDetailGachaClaims,
			PermissionsEnum::ReadListGachaItems,
			PermissionsEnum::ReadDetailGachaItems,
			PermissionsEnum::CreateGachaItems,
			PermissionsEnum::DeleteGachaItems,
			PermissionsEnum::UpdateGachaItems,
			PermissionsEnum::ReadDetailGachaRolls,
			PermissionsEnum::CreateGachaRolls,
			PermissionsEnum::ExecuteGachaRolls,
			PermissionsEnum::DeleteGachaRolls,
			
			// Mentor permissions
			PermissionsEnum::ReadListMentors,
			PermissionsEnum::ReadDetailMentors,
			PermissionsEnum::RegisterMentors,
			PermissionsEnum::ReadOwnMentorProfile,
			PermissionsEnum::UpdateOwnMentorProfile,
			PermissionsEnum::ReadOwnMentorStatus,
			PermissionsEnum::UpdateMentors,
			PermissionsEnum::VerifyMentors,
			PermissionsEnum::DeleteMentors,
			
			// Administrator permissions
			PermissionsEnum::ManageAllUsers,
			PermissionsEnum::ManageAllRoles,
			PermissionsEnum::ManageAllPermissions,
			PermissionsEnum::ManageAllTeams,
			PermissionsEnum::ViewAllSensitiveData,
			PermissionsEnum::AccessAdminDashboard,
		]
	}
}
