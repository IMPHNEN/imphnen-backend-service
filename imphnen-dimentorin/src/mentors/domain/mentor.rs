use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct MentorEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub industries: Vec<String>,
    pub expertise: Vec<String>,
    pub languages: Vec<String>,
    pub current_company: String,
    pub current_role: String,
    pub years_of_experience: i32,
    pub topics_of_interest: Vec<String>,
    pub preferred_mentee_level: Vec<String>,
    pub preferred_mentoring_formats: Vec<String>,
    pub availability_commitment: String,
    pub mentoring_rate: f64,
    pub status: String,
    pub is_deleted: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
