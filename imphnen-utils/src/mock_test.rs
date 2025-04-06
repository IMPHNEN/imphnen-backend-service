use crate::AppState;
use surrealdb::{
	Surreal,
	engine::{local::Mem, remote::ws::Ws},
	opt::auth::Root,
};

pub async fn create_mock_app_state() -> AppState {
	let db_mem = Surreal::new::<Mem>(()).await.unwrap();
	let db_ws = Surreal::new::<Ws>("localhost:8000").await.unwrap();
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
