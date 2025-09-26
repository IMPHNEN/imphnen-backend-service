//! Axum server initialization utilities.
//!
//! This module provides utilities for initializing and running an Axum web server
//! with SurrealDB connections for both WebSocket and in-memory databases.

use crate::{surrealdb_init_mem, surrealdb_init_ws, SurrealMemClient, SurrealWsClient};
use axum::{Router, serve};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;
use crate::environment::ENV;

/// Initialize and start the Axum server with SurrealDB connections.
///
/// This function sets up both WebSocket and in-memory SurrealDB connections,
/// builds the router using the provided function, and starts the server.
///
/// # Arguments
/// * `router_fn` - A function that takes SurrealDB clients and returns a Router
///
/// # Panics
/// This function will panic if:
/// - SurrealDB initialization fails
/// - TCP listener binding fails
///
/// # Example
/// ```no_run
/// use axum::Router;
/// use imphnen_libs::{axum_init, SurrealWsClient, SurrealMemClient};
///
/// async fn create_router(ws: SurrealWsClient, mem: SurrealMemClient) -> Router {
///     Router::new()
///         // Add your routes here
/// }
///
/// #[tokio::main]
/// async fn main() {
///     axum_init(create_router).await;
/// }
/// ```
pub async fn axum_init<F, Fut>(router_fn: F)
where
    F: FnOnce(SurrealWsClient, SurrealMemClient) -> Fut,
    Fut: Future<Output = Router>,
{
    let env = &ENV;

    // Initialize SurrealDB connections
    log::info!("Initializing SurrealDB connections...");
    let surrealdb_ws = surrealdb_init_ws()
        .await
        .expect("Failed to initialize SurrealDB WebSocket connection");

    let surrealdb_mem = surrealdb_init_mem()
        .await
        .expect("Failed to initialize SurrealDB in-memory connection");

    log::info!("SurrealDB connections established successfully");

    // Build the router
    let router = router_fn(surrealdb_ws, surrealdb_mem).await;

    // Start the server
    let port = env.port;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    log::info!("Starting server on {}", addr);

    let listener = TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|e| {
            log::error!("Failed to bind to address {}: {}", addr, e);
            panic!("Server binding failed: {}", e);
        });

    log::info!("Server listening on {}", addr);

    if let Err(err) = serve(listener, router).await {
        log::error!("Server encountered an error: {}", err);
        panic!("Server failed: {}", err);
    }
}
