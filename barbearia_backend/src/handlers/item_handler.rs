use axum::{Json, extract::{Path, State}, http::StatusCode};
use sqlx::PgPool;
use crate::models::item::{CreateItemRequest, UpdateItemRequest, ItemResponse};
use crate::response::ApiResponse;

// Create
pub async fn create_item(State(pool) : State<PgPool>, Json(payload) : Json<CreateItemRequest>) -> (StatusCode, Json<ApiResponse<ItemResponse>>){
    
    let result = sqlx::query_as::<_, (i32, String, Option<String>, f64, i32)> (
        "INSERT INTO item (nome, descricao, estoque_atual)
        VALUES ($1, $2, $3, $4)
        RETURNING id, name, description, price, stock"
    )
    .bind(&payload.name)
    .bind(&payload.description)
    .bind(payload.price)
    .bind(payload.stock)
    .fetch_one(&pool)
    .await;

    match result {
        Ok((id, name, description, price, stock)) => {
            let item_response = ItemResponse { id, name, description, price, stock};
            let
        }
    }
    
    todo!()
}