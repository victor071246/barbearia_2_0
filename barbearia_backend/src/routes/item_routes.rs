use axum:: {
    routing::{get, post, put, delete},
    Router,
};
use sqlx::PgPool;
use crate::handlers::item_handler::{
    create_item,
    get_all_items,
    get_item_by_id,
    update_item
}