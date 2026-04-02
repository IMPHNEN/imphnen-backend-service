use axum::{
    middleware::from_fn,
    routing::{delete, get, put},
    Extension, Router,
};
use sqlx::PgPool;
use std::sync::Arc;

use crate::{
    common::qr_jwt::QrJwtService,
    middleware::qr_auth::qr_auth_middleware,
    users::{
        application::user_service::QrUserServiceImpl,
        domain::{repository::UserRepository, service::QrUserService},
        infrastructure::{
            http::handlers::{
                delete_user_handler, get_me_handler, list_users_handler, update_me_handler,
                update_role_handler,
            },
            persistence::postgres_user_repository::PostgresUserRepository,
        },
    },
};

pub fn qr_users_routes(pool: Arc<PgPool>, jwt: Arc<QrJwtService>) -> Router {
    let repo: Arc<dyn UserRepository> = Arc::new(PostgresUserRepository::new(pool.clone()));
    let service: Arc<dyn QrUserService> = Arc::new(QrUserServiceImpl::new(repo));

    Router::new()
        .route("/users/me", get(get_me_handler).put(update_me_handler))
        .route("/users", get(list_users_handler))
        .route("/users/:id/role", put(update_role_handler))
        .route("/users/:id", delete(delete_user_handler))
        .layer(Extension(service))
        .layer(Extension(jwt.clone()))
        .layer(Extension(pool))
        .layer(from_fn(qr_auth_middleware))
}
