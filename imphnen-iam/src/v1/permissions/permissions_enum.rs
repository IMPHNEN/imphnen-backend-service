use std::fmt;

use strum_macros::EnumIter;

#[derive(Debug, Clone, PartialEq, Eq, EnumIter)]
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
	ReadListTeams,
	ReadDetailTeams,
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
		};
		write!(f, "{permission_str}")
	}
}

impl PermissionsEnum {
	pub fn id(&self) -> &'static str {
		match self {
			PermissionsEnum::ReadListUsers => "7c15e31d-36e2-49f9-97db-138c03fb0cf6",
			PermissionsEnum::ReadDetailUsers => "319ee593-ff0a-4f29-bbaf-9feb3174a3a6",
			PermissionsEnum::CreateUsers => "023e2dfe-93c3-4008-94a8-b5dff403f73b",
			PermissionsEnum::DeleteUsers => "96df0689-2ae9-4894-bf00-837c19415e5c",
			PermissionsEnum::UpdateUsers => "98b3dc4c-0124-461f-afcd-166637c5e6e8",
			PermissionsEnum::ActivateUsers => "4da8b434-89f9-4d91-85ae-eebd63cdbeda",
			PermissionsEnum::ReadListRoles => "9164ca6e-c7e3-4238-a15f-f36ab9577e7e",
			PermissionsEnum::ReadDetailRoles => "73888d18-b3e9-4f62-95a5-ba2c0d69fccb",
			PermissionsEnum::CreateRoles => "319ee593-ff0a-4f29-bbaf-9feb3174a3a2",
			PermissionsEnum::DeleteRoles => "35b0d992-65c8-4b62-b030-e6e0320e4048",
			PermissionsEnum::UpdateRoles => "a00d5608-4c48-4542-845c-dfe004687022",
			PermissionsEnum::ReadListPermissions => "8195eeb8-e64f-4172-aa57-596492c84a72",
			PermissionsEnum::ReadDetailPermissions => {
				"dad435cf-042c-41bd-a946-cea61ed2ffbc"
			}
			PermissionsEnum::CreatePermissions => "0269ed71-0ae0-4c43-ad29-e3d861d8f9a0",
			PermissionsEnum::DeletePermissions => "b2dc3928-86ba-4c59-a03d-0b57d5183ebc",
			PermissionsEnum::UpdatePermissions => "299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b",
			PermissionsEnum::CreateGachaClaims => "f41d53ce-4f88-4bb6-b9b4-5e3a8c38d962",
			PermissionsEnum::ReadDetailGachaClaims => {
				"c1c3d6c2-19fb-4b70-b58c-c19f2e8cfc79"
			}
			PermissionsEnum::ReadListGachaItems => "fa6eb842-0a61-40c2-9c24-b226ad975037",
			PermissionsEnum::ReadDetailGachaItems => {
				"9c7857d7-b5ae-4688-923d-ef5572e9bc8b"
			}
			PermissionsEnum::CreateGachaItems => "cf063be1-4d71-489e-b9fb-1c08c65f396c",
			PermissionsEnum::DeleteGachaItems => "46f8c6cf-ea0c-4c90-860c-69e2e65f7eb1",
			PermissionsEnum::UpdateGachaItems => "2d0cf4ae-56ae-4714-a12e-655cfc3d9eb2",
			PermissionsEnum::ReadDetailGachaRolls => {
				"53d6483a-04cd-4667-8792-2d0cc8e2d343"
			}
			PermissionsEnum::CreateGachaRolls => "18e36c63-fcb7-4877-b911-c5aa611e878f",
			PermissionsEnum::ExecuteGachaRolls => "14c6a1cd-5c63-4643-89b5-b1a5f9920cc0",
			PermissionsEnum::DeleteGachaRolls => "12345678-ABCD-EFAB-CDEF-0123456789AB",
			PermissionsEnum::ReadListMentors => "a1b2c3d4-5e6f-7890-abcd-ef1234567890",
			PermissionsEnum::ReadDetailMentors => "b2c3d4e5-6f78-9012-bcde-f23456789012",
			PermissionsEnum::ReadListTeams => "e1f2g3h4-5i6j-7k8l-9m0n-op1q2r3s4t5u",
			PermissionsEnum::ReadDetailTeams => "f2g3h4i5-6j7k-8l9m-0n1o-pq2r3s4t5u6",
			PermissionsEnum::RegisterMentors => "c3d4e5f6-7890-1234-cdef-345678901234",
			PermissionsEnum::ReadOwnMentorProfile => {
				"d4e5f6a7-8901-2345-def0-456789012345"
			}
			PermissionsEnum::UpdateOwnMentorProfile => {
				"e5f6a7b8-9012-3456-ef01-567890123456"
			}
			PermissionsEnum::ReadOwnMentorStatus => "f6a7b8c9-0123-4567-f012-678901234567",
			PermissionsEnum::UpdateMentors => "a7b8c9d0-1234-5678-0123-789012345678",
			PermissionsEnum::VerifyMentors => "b8c9d0e1-2345-6789-1234-890123456789",
			PermissionsEnum::DeleteMentors => "c9d0e1f2-3456-7890-2345-901234567890",
		}
	}
}
