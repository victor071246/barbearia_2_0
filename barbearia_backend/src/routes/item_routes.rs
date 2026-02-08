use axum::{
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::handlers::item_handler::{
    create_item,
    get_all_items,
    get_item_by_id,
    update_item,
    delete_item,
};

pub fn item_routes(pool: PgPool) -> Router {
    Router::new()
    .route("/items", post(create_item))
    .route("/items", get(get_all_items))
    .route("/items/:id", get(get_item_by_id))
    .route("/items/:id", delete(delete_item))
    .with_state(pool)
}