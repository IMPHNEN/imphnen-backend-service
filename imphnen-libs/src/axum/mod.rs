use crate::{Env, surrealdb_init_mem, surrealdb_init_ws};
use axum::{Router, serve};
use imphnen_entities::{SurrealMemClient, SurrealWsClient};
use log::{debug, error, info};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn axum_init<F, Fut>(router_fn: F)
where
	F: FnOnce(SurrealWsClient, SurrealMemClient) -> Fut,
	Fut: Future<Output = Router>,
{
	let env = Env::new();
	info!("Environment loaded with port: {}", env.port);

	let surrealdb_ws = surrealdb_init_ws().await.expect("Failed surrealdb ws");
	info!("SurrealDB WS client initialized");

	let surrealdb_mem = surrealdb_init_mem().await.expect("Failed surrealdb mem");
	info!("SurrealDB MEM client initialized");

	let router = router_fn(surrealdb_ws, surrealdb_mem).await;
	debug!("Router created successfully");

	let port = env.port;
	let addr = SocketAddr::from(([0, 0, 0, 0], port));
	let listener = TcpListener::bind(&addr).await.unwrap();
	info!("Listening on http://{}", addr);

	match serve(listener, router).await {
		Ok(_) => info!("Server stopped gracefully."),
		Err(err) => error!("Server encountered an error: {}", err),
	}
}
