use imphnen_libs::enviroment::load_env;
use imphnen_utils::{get_iso_date, hash_password, Env};
use serde_json::json;
use std::error::Error;
use surrealdb::opt::auth::Root;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	load_env();
	let env = Env::new();
	use surrealdb::engine::any;
	let db = any::connect(&env.surrealdb_url).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace)
		.use_db(env.surrealdb_dbname)
		.await?;

	db.query("DELETE type::thing('app_mentors', $id)")
		.bind(("id", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
		.await?;

	db.query("DELETE type::thing('app_users', $id)")
		.bind(("id", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
		.await?;

	use surrealdb::sql::Thing;
	db.query("CREATE type::thing('app_users', $id) SET fullname = $fullname, email = $email, password = $password, avatar = $avatar, phone_number = $phone_number, is_active = $is_active, is_deleted = $is_deleted, mentor_id = $mentor_id, gender = $gender, birthdate = $birthdate, role = $role, created_at = $created_at, updated_at = $updated_at")
	   .bind(("id", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
	   .bind(("fullname", "Mentor User"))
	   .bind(("email", "mentor@example.com"))
	   .bind(("password", hash_password("password").unwrap()))
	   .bind(("avatar", Option::<String>::None))
	   .bind(("phone_number", "081234567890"))
	   .bind(("is_active", true))
	   .bind(("is_deleted", false))
	   .bind(("mentor_id", Option::<Thing>::None))
	   .bind(("gender", "male"))
	   .bind(("birthdate", "1990-05-15"))
	   .bind(("role", Thing::from(("app_roles", "3b9f8c4e-6a2d-4f8a-9a12-2d6f8b3c4e5a"))))
	   .bind(("created_at", get_iso_date()))
	   .bind(("updated_at", get_iso_date()))
	   .await?;

	db.query("CREATE type::thing('app_mentors', $id) SET user_id = $user_id, legal_name = $legal_name, identity_document_url = $identity_document_url, phone_for_verification = $phone_for_verification, bio = $bio, linkedin_url = $linkedin_url, github_url = $github_url, cv_url = $cv_url, industries = $industries, expertise = $expertise, languages = $languages, current_company = $current_company, current_role = $current_role, years_of_experience = $years_of_experience, topics_of_interest = $topics_of_interest, preferred_mentee_level = $preferred_mentee_level, preferred_mentoring_formats = $preferred_mentoring_formats, availability_commitment = $availability_commitment, mentoring_rate = $mentoring_rate, status = $status, is_deleted = $is_deleted, created_at = $created_at, updated_at = $updated_at, email = $email")
	   .bind(("id", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
	   .bind(("user_id", Thing::from(("app_users", "e6f78d23-83bf-5c2b-bcd4-001345678901"))))
	   .bind(("legal_name", "Mentor User"))
	   .bind(("identity_document_url", "https://example.com/ktp.jpg"))
	   .bind(("phone_for_verification", "081234567890"))
	   .bind(("bio", "Saya adalah mentor backend Rust dengan pengalaman 5 tahun dalam pengembangan aplikasi backend yang scalable dan performant."))
	   .bind(("linkedin_url", "https://linkedin.com/in/mentor"))
	   .bind(("github_url", "https://github.com/mentor"))
	   .bind(("cv_url", Option::<String>::None))
	   .bind(("industries", vec!["Software", "Education"]))
	   .bind(("expertise", vec!["Rust", "Microservices"]))
	   .bind(("languages", vec!["Indonesian", "English"]))
	   .bind(("current_company", "PT Contoh"))
	   .bind(("current_role", "Senior Backend Engineer"))
	   .bind(("years_of_experience", 5))
	   .bind(("topics_of_interest", vec!["Rust Programming", "Backend Development"]))
	   .bind(("preferred_mentee_level", vec!["beginner", "intermediate"]))
	   .bind(("preferred_mentoring_formats", vec!["online", "offline"]))
	   .bind(("availability_commitment", "2 jam per minggu untuk mentoring online dan offline"))
	   .bind(("mentoring_rate", json!({
		   "amount": 100000,
		   "currency": "IDR",
		   "per_duration": "hour"
	   })))
	   .bind(("status", "verified"))
	   .bind(("is_deleted", false))
	   .bind(("created_at", get_iso_date()))
	   .bind(("updated_at", get_iso_date()))
       .bind(("email", "mentor@example.com"))
	   .await?;
	println!("Mentor created successfully!");
	println!("Updating user with mentor_id...");

	db.query("UPDATE type::thing('app_users', $id) SET mentor_id = $mentor_id")
		.bind(("id", "e6f78d23-83bf-5c2b-bcd4-001345678901"))
		.bind((
			"mentor_id",
			Thing::from(("app_mentors", "e6f78d23-83bf-5c2b-bcd4-001345678901")),
		))
		.await?;
	println!("User updated with mentor_id successfully!");

	println!("✅ Inserted mentor user: mentor@example.com");
	println!("✅ Mentor user seeded");
	Ok(())
}
