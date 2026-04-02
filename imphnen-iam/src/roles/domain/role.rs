use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct RoleEntity {
	pub id: Uuid,
	pub name: String,
	pub description: String,
	pub is_system_role: bool,
	pub is_default: bool,
	pub permissions: Vec<String>,
	pub created_at: Option<String>,
	pub updated_at: Option<String>,
	pub deleted_at: Option<String>,
}
