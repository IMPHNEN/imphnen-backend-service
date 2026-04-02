use super::definitions::PermissionsEnum;

impl PermissionsEnum {
	pub fn id(&self) -> String {
		match self {
			PermissionsEnum::ReadListUsers => {
				"7c15e31d-36e2-49f9-97db-138c03fb0cf6".to_string()
			}
			PermissionsEnum::ReadDetailUsers => {
				"319ee593-ff0a-4f29-bbaf-9feb3174a3a6".to_string()
			}
			PermissionsEnum::CreateUsers => {
				"023e2dfe-93c3-4008-94a8-b5dff403f73b".to_string()
			}
			PermissionsEnum::DeleteUsers => {
				"96df0689-2ae9-4894-bf00-837c19415e5c".to_string()
			}
			PermissionsEnum::UpdateUsers => {
				"98b3dc4c-0124-461f-afcd-166637c5e6e8".to_string()
			}
			PermissionsEnum::ActivateUsers => {
				"4da8b434-89f9-4d91-85ae-eebd63cdbeda".to_string()
			}

			PermissionsEnum::ReadListRoles => {
				"9164ca6e-c7e3-4238-a15f-f36ab9577e7e".to_string()
			}
			PermissionsEnum::ReadDetailRoles => {
				"73888d18-b3e9-4f62-95a5-ba2c0d69fccb".to_string()
			}
			PermissionsEnum::CreateRoles => {
				"319ee593-ff0a-4f29-bbaf-9feb3174a3a2".to_string()
			}
			PermissionsEnum::DeleteRoles => {
				"35b0d992-65c8-4b62-b030-e6e0320e4048".to_string()
			}
			PermissionsEnum::UpdateRoles => {
				"a00d5608-4c48-4542-845c-dfe004687022".to_string()
			}

			PermissionsEnum::ReadListPermissions => {
				"8195eeb8-e64f-4172-aa57-596492c84a72".to_string()
			}
			PermissionsEnum::ReadDetailPermissions => {
				"dad435cf-042c-41bd-a946-cea61ed2ffbc".to_string()
			}
			PermissionsEnum::CreatePermissions => {
				"0269ed71-0ae0-4c43-ad29-e3d861d8f9a0".to_string()
			}
			PermissionsEnum::DeletePermissions => {
				"b2dc3928-86ba-4c59-a03d-0b57d5183ebc".to_string()
			}
			PermissionsEnum::UpdatePermissions => {
				"299cb4d5-6556-4cc9-b6c1-32e6d31e0f9b".to_string()
			}

			PermissionsEnum::CreateGachaClaims => {
				"f41d53ce-4f88-4bb6-b9b4-5e3a8c38d962".to_string()
			}
			PermissionsEnum::ReadDetailGachaClaims => {
				"c1c3d6c2-19fb-4b70-b58c-c19f2e8cfc79".to_string()
			}
			PermissionsEnum::ReadListGachaItems => {
				"fa6eb842-0a61-40c2-9c24-b226ad975037".to_string()
			}
			PermissionsEnum::ReadDetailGachaItems => {
				"9c7857d7-b5ae-4688-923d-ef5572e9bc8b".to_string()
			}
			PermissionsEnum::CreateGachaItems => {
				"cf063be1-4d71-489e-b9fb-1c08c65f396c".to_string()
			}
			PermissionsEnum::DeleteGachaItems => {
				"46f8c6cf-ea0c-4c90-860c-69e2e65f7eb1".to_string()
			}
			PermissionsEnum::UpdateGachaItems => {
				"2d0cf4ae-56ae-4714-a12e-655cfc3d9eb2".to_string()
			}
			PermissionsEnum::ReadDetailGachaRolls => {
				"53d6483a-04cd-4667-8792-2d0cc8e2d343".to_string()
			}
			PermissionsEnum::CreateGachaRolls => {
				"18e36c63-fcb7-4877-b911-c5aa611e878f".to_string()
			}
			PermissionsEnum::ExecuteGachaRolls => {
				"14c6a1cd-5c63-4643-89b5-b1a5f9920cc0".to_string()
			}
			PermissionsEnum::DeleteGachaRolls => {
				"12345678-ABCD-EFAB-CDEF-0123456789AB".to_string()
			}

			PermissionsEnum::ReadListMentors => {
				"a1b2c3d4-5e6f-7890-abcd-ef1234567890".to_string()
			}
			PermissionsEnum::ReadDetailMentors => {
				"b2c3d4e5-6f78-9012-bcde-f23456789012".to_string()
			}
			PermissionsEnum::RegisterMentors => {
				"c3d4e5f6-7890-1234-cdef-345678901234".to_string()
			}
			PermissionsEnum::ReadOwnMentorProfile => {
				"d4e5f6a7-8901-2345-def0-456789012345".to_string()
			}
			PermissionsEnum::UpdateOwnMentorProfile => {
				"e5f6a7b8-9012-3456-ef01-567890123456".to_string()
			}
			PermissionsEnum::ReadOwnMentorStatus => {
				"f6a7b8c9-0123-4567-f012-678901234567".to_string()
			}
			PermissionsEnum::UpdateMentors => {
				"a7b8c9d0-1234-5678-0123-789012345678".to_string()
			}
			PermissionsEnum::VerifyMentors => {
				"b8c9d0e1-2345-6789-1234-890123456789".to_string()
			}
			PermissionsEnum::DeleteMentors => {
				"c9d0e1f2-3456-7890-2345-901234567890".to_string()
			}

			PermissionsEnum::ManageAllUsers => {
				"d0e1f2a3-4567-8901-2345-0123456789ab".to_string()
			}
			PermissionsEnum::ManageAllRoles => {
				"e1f2a3b4-5678-9012-3456-1234567890ab".to_string()
			}
			PermissionsEnum::ManageAllPermissions => {
				"f2a3b4c5-6789-0123-4567-2345678901ab".to_string()
			}
			PermissionsEnum::ViewAllSensitiveData => {
				"b4c5d6e7-8901-2345-6789-4567890123ab".to_string()
			}
			PermissionsEnum::AccessAdminDashboard => {
				"c5d6e7f8-9012-3456-7890-5678901234ab".to_string()
			}
			PermissionsEnum::Administrator => {
				"d6e7f8a9-0123-4567-8901-6789012345ab".to_string()
			}
		}
	}

	pub fn all() -> Vec<PermissionsEnum> {
		vec![
			PermissionsEnum::ReadListUsers,
			PermissionsEnum::ReadDetailUsers,
			PermissionsEnum::CreateUsers,
			PermissionsEnum::DeleteUsers,
			PermissionsEnum::UpdateUsers,
			PermissionsEnum::ActivateUsers,
			PermissionsEnum::ReadListRoles,
			PermissionsEnum::ReadDetailRoles,
			PermissionsEnum::CreateRoles,
			PermissionsEnum::DeleteRoles,
			PermissionsEnum::UpdateRoles,
			PermissionsEnum::ReadListPermissions,
			PermissionsEnum::ReadDetailPermissions,
			PermissionsEnum::CreatePermissions,
			PermissionsEnum::DeletePermissions,
			PermissionsEnum::UpdatePermissions,
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
			PermissionsEnum::ReadListMentors,
			PermissionsEnum::ReadDetailMentors,
			PermissionsEnum::RegisterMentors,
			PermissionsEnum::ReadOwnMentorProfile,
			PermissionsEnum::UpdateOwnMentorProfile,
			PermissionsEnum::ReadOwnMentorStatus,
			PermissionsEnum::UpdateMentors,
			PermissionsEnum::VerifyMentors,
			PermissionsEnum::DeleteMentors,
			PermissionsEnum::ManageAllUsers,
			PermissionsEnum::ManageAllRoles,
			PermissionsEnum::ManageAllPermissions,
			PermissionsEnum::ViewAllSensitiveData,
			PermissionsEnum::AccessAdminDashboard,
			PermissionsEnum::Administrator,
		]
	}
}
