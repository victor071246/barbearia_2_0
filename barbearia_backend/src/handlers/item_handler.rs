use axum::{Json, extract::{Path, State}, http::StatusCode};
use sqlx::PgPool;
use crate::models::item::{CreateItemRequest, UpdateItemRequest, ItemResponse};
use crate::response::ApiResponse;

// CREATE
pub async fn create_item(State(pool): State<PgPool>, Json(payload): Json<CreateItemRequest>) -> (StatusCode, Json<ApiResponse<ItemResponse>>) {
    let result = sqlx::query_as::<_, ItemResponse>(
        "INSERT INTO items (nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by"
    )
    .bind(&payload.nome)
    .bind(&payload.descricao)
    .bind(payload.preco)
    .bind(&payload.image_url)
    .bind(&payload.tipo)
    .bind(&payload.estoque_atual)
    .bind(payload.estoque_minimo)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(item) => {
            let response = ApiResponse {
                message: "Item created successfuly".to_string(),
                data: Some(item),
            };
            (StatusCode::CREATED, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to create item: {}", e),
                data: None,
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

//GET ALL
pub async fn get_all_items(State(pool): State<PgPool>) -> (StatusCode, Json<ApiResponse<Vec<ItemResponse>>>) {
    let result = sqlx::query_as::<_, ItemResponse> (
        "SELECT id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by
        FROM items"
    )
    .fetch_all(&pool)
    .await;

    match result {
        Ok(items) => {
            let response = ApiResponse {
                message: "Items found".to_string(),
                data: Some(items),
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to get items: {}", e),
                data: None
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

//GET ONE
pub async fn get_item_by_id(State(pool): State<PgPool>, Path(id): Path<i32>) -> (StatusCode, Json<ApiResponse<ItemResponse>>) {
    let result = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by FROM items WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(item) => {
            let response = ApiResponse {
                message: "Item found".to_string(),
                data: Some(item)
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Item not found {e}"),
                data: None,
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

// UPDATE
pub async fn update_item(State(pool): State<PgPool>, Path(id): Path<i32>, Json(payload): Json<UpdateItemRequest>) -> (StatusCode, Json<ApiResponse<ItemResponse>>) {

    // Busca item atual
    let current = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by
        FROM items WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    let current_item = match current {
        Ok(item) => item,
        Err(e) => {
            let response = ApiResponse {
                message: format!("Item not found: {e}"),
                data: None,
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
    };

    // Atualiza campos (usa valor novo ou mantém atual)
    let new_nome = payload.nome.unwrap_or(current_item.nome);
    let new_descricao = payload.descricao.or(current_item.descricao);
    let new_preco = payload.preco.unwrap_or(current_item.preco);
    let new_image_url = payload.image_url.or(current_item.image_url);
    let new_tipo = payload.tipo.or(current_item.tipo);
    let new_estoque_atual = payload.estoque_atual.unwrap_or(current_item.estoque_atual);
    let new_estoque_minimo = payload.estoque_minimo.unwrap_or(current_item.estoque_minimo);

    let result = sqlx::query_as::<_, ItemResponse>(
        "UPDATE items
        SET nome = $1, descricao = $2, preco = $3, image_url = $4, tipo = $5, estoque_minimo = $7, updated_at = NOW()
        WHERE id = $8
        RETURNING id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by"
    )
    .bind(&new_nome)
    .bind(&new_descricao)
    .bind(new_preco)
    .bind(&new_image_url)
    .bind(&new_tipo)
    .bind(new_estoque_atual)
    .bind(new_estoque_minimo)
    .bind(id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(item) => {
            let response = ApiResponse {
                message: "Item updated successfuly".to_string(),
                data: Some(item)
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to update item: {}", e),
                data: None 
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }

}

// DELETE
pub async fn delete_item(
    State(pool) : State<PgPool>,
    Path(id): Path<i32>
) -> (StatusCode, Json<ApiResponse<()>>) {
    let result = sqlx::query("DELETE FROM items WHERE id = $1")
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let response = ApiResponse {
                    message: "Item deleted successfuly".to_string(),
                    data: None
                };
                (StatusCode::OK, Json(response))
            } else {
                let response = ApiResponse {
                    message: "Item not found".to_string(),
                    data: None
                };
                (StatusCode::NOT_FOUND, Json(response))
            }
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to delete item: {}", e),
                data: None
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}