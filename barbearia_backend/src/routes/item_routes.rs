use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware
};
use sqlx::PgPool;
use crate::handlers::item_handler::{
    create_item,
    get_all_items,
    get_item_by_id,
    update_item,
    delete_item,
};
use crate::middleware::auth_middleware::auth_middleware;

pub fn item_routes(pool: PgPool) -> Router {
    let protected = Router::new()
    .route("/items", post(create_item))
    .route("/items/:id", put(update_item))
    .route("/items/:id", delete(delete_item))
    .layer(middleware::from_fn(auth_middleware));

    let public = Router::new()
    .route("/items", get(get_all_items))
    .route("/items/:id", get(get_item_by_id));

    Router::new()
        .merge(protected)
        .merge(public)
        .with_state(pool)
}