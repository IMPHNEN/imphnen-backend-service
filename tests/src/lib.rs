use ::surrealdb::Uuid;
pub use imphnen_entities::*;
pub use imphnen_iam::*;

#[cfg(test)]
pub mod iam;

pub fn generate_unique_email(prefix: &str) -> String {
	format!("{}_{}@example.com", prefix, Uuid::new_v4())
}

pub async fn get_role_id(state: &crate::AppState) -> String {
	let repo = RolesRepository::new(state);
	if let Ok(existing) = repo.query_role_by_name("User".into()).await {
		return existing.id;
	}
	let _ = repo
		.query_create_role(RolesRequestCreateDto {
			name: "User".into(),
			permissions: vec![],
		})
		.await;
	repo
		.query_role_by_name("User".into())
		.await
		.expect("Role not found after creation")
		.id
}
