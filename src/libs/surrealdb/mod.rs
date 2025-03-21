use super::Env;
use crate::SurrealClient;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

pub mod resource;
pub use resource::*;

pub async fn surrealdb_init() -> Result<SurrealClient> {
	let env = Env::new();
	let db = Surreal::<Client>::init();
	db.connect::<Ws>(env.surrealdb_url.clone()).await?;
	db.signin(Root {
		username: &env.surrealdb_username,
		password: &env.surrealdb_password,
	})
	.await?;
	db.use_ns(env.surrealdb_namespace.clone())
		.use_db(env.surrealdb_dbname.clone())
		.await?;
	Ok(db)
}
