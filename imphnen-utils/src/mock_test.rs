use crate::AppState;
use imphnen_libs::enviroment::load_env;
use surrealdb::engine::any;
use surrealdb::{Surreal, engine::local::Mem, opt::auth::Root};
use super::Env;

pub async fn create_mock_app_state() -> AppState {
	load_env();
	let env = Env::new();
	let db_mem = Surreal::new::<Mem>(()).await.unwrap();
	let db_ws = any::connect(&env.surrealdb_url).await.unwrap();
	db_mem.use_ns("test").use_db("test").await.unwrap();
	db_ws
		.signin(Root {
			username: "root",
			password: "root",
		})
		.await
		.unwrap();
	db_ws.use_ns("test").use_db("test").await.unwrap();

	AppState {
		surrealdb_mem: db_mem,
		surrealdb_ws: db_ws,
	}
}

pub async fn cleanup_db() {
	let app_state = create_mock_app_state().await;
	let _ = app_state
		.surrealdb_mem
		.query(
			r#"
    REMOVE TABLE app_users;
    REMOVE TABLE app_roles;
    REMOVE TABLE app_users_cache;
    REMOVE TABLE app_otp_cache;
"#,
		)
		.await;
	let _ = app_state
		.surrealdb_ws
		.query(
			r#"
    REMOVE TABLE app_users;
    REMOVE TABLE app_roles;
    REMOVE TABLE app_users_cache;
    REMOVE TABLE app_otp_cache;
"#,
		)
		.await;
}
