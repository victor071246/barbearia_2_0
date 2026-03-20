use axum::{routing::get, Router};
use sqlx::PgPool;
use crate::handlers::health_handler;

pub fn health_routes(pool: PgPool) -> Router {
    Router::new().route("/health", get(health_handler::health)).with_state(pool)
}