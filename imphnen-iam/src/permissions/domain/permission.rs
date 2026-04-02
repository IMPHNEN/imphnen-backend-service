use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct PermissionEntity {
    pub id: Uuid,
    pub name: String,
    pub is_deleted: bool,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
