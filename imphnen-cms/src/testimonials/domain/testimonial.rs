use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct TestimonialEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub user_fullname: String,
    pub role: String,
    pub content: String,
    pub is_deleted: bool,
    pub created_at: String,
    pub updated_at: String,
}
