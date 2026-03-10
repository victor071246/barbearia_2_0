use axum::{Router, routing::{Route, get, post}};
use sqlx::PgPool;
use crate::handlers::auth_handler;

pub fn auth_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/auth/login", post(auth_handler::login))
        .route("/auth/verify", get(auth_handler::verify_auth))
        .with_state(pool)
}