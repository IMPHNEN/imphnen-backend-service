use crate::{
	surrealdb_init_mem, surrealdb_init_ws, Env, SurrealMemClient, SurrealWsClient,
};
use axum::{serve, Router};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: FnOnce(SurrealWsClient, SurrealMemClient) -> Fut,
	Fut: Future<Output = Router>,
{
	let env = Env::new();
	let surrealdb_ws = surrealdb_init_ws().await.expect("Failed surrealdb ws");
	let surrealdb_mem = surrealdb_init_mem().await.expect("Failed surrealdb mem");
	let router = router_fn(surrealdb_ws, surrealdb_mem).await;
	let port = env.port;
	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	let listener = TcpListener::bind(&addr).await.unwrap();
	println!("Listening on http://{}", addr);
	match serve(listener, router).await {
		Ok(_) => println!("Server stopped gracefully."),
		Err(err) => println!("Server encountered an error: {}", err),
	}
}
