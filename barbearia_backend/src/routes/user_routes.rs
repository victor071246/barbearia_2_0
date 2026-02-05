use axum::{
    routing::{get, post, put, delete},
    Router
};
use sqlx::PgPool;
use crate::handlers::user_handler;

pub fn user_routes(pool: PgPool) -> Router {
    Router::new()
        .route("/users", post(user_handler::create_user))
        .route("/users", get(user_handler::get_all_users))
        .route("/users/:id", get(user_handler::get_user_by_id))
        .route("/users/:id", put(user_handler::update_user))
        .route("/users/:id", delete(user_handler::delete_user))
        .with_state(pool)
}