use axum::{Json, extract::{Path, State}, http::StatusCode};
use sqlx::PgPool;
use crate::models::item::{ItemResponse};
use crate::response::ApiResponse;
use axum::extract::Multipart;
use crate::middleware::upload_middleware::extract_image;

// CREATE
pub async fn create_item(
    State(pool): State<PgPool>,
    mut multipart: Multipart
) -> (StatusCode, Json<ApiResponse<ItemResponse>>) {
    let mut nome = String::new();
    let mut descricao: Option<String> = None;
    let mut preco: f64 = 0.0;
    let mut tipo: Option<String> = None;
    let mut estoque_atual: i32 = 0;
    let mut estoque_minimo: i32 = 0;
    let mut image_url: Option<String> = None;

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name().unwrap_or(""){
            "nome" => nome = field.text().await.unwrap_or_default(),
            "descricao" => descricao = Some(field.text().await.unwrap_or_default()),
            "preco" => preco = field.text().await.unwrap_or_default().parse().unwrap_or(0.0),
            "tipo" => tipo = Some(field.text().await.unwrap_or_default()),
            "estoque_atual" => estoque_atual = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "estoque_minimo" => estoque_minimo = field.text().await.unwrap_or_default().parse().unwrap_or(0),
            "image" => image_url = extract_image(field).await,
            _ => {}
        }
    }

    let result = sqlx::query_as::<_, ItemResponse>(
        "INSERT INTO items (nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by"
    )
    .bind(&nome)
    .bind(&descricao)
    .bind(preco)
    .bind(&image_url)
    .bind(&tipo)
    .bind(estoque_atual)
    .bind(estoque_minimo)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(item) => (StatusCode::CREATED, Json(ApiResponse {
            message: "Item created successfully".to_string(),
            data: Some(item),
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse {
            message: format!("Failed to create item: {}", e),
            data: None,
        }))
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

pub async fn get_items_by_category( State(pool): State<PgPool>, Path(category): Path<String>) -> (StatusCode, Json<ApiResponse<Vec<ItemResponse>>>) {
    let result = sqlx::query_as::<_, ItemResponse>(
        "SELECT id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo FROM items WHERE tipo = $1"
    ).bind(&category).fetch_all(&pool).await;

    match result {
        Ok(items) => {
            let response = ApiResponse {
                message: "Items found".to_string(),
                data: Some(items)
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to get items: {}", e),
                data: None,
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
pub async fn update_item(
    State(pool): State<PgPool>,
    Path(id): Path<i32>,
    mut multipart: Multipart
) -> (StatusCode, Json<ApiResponse<ItemResponse>>) {

    //Busca item atual
    let current = sqlx::query_as::<_, ItemResponse> (
        "SELECT id, nome, preco, descricao, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by
            FROM items WHERE id = $1"
    )
    .bind(id)
    .fetch_one(&pool)
    .await;

    let current_item = match current{
        Ok(item) => item,
        Err(e) => return (StatusCode::NOT_FOUND, Json(ApiResponse {
            message: format!("Item not foudn: {e}"),
            data: None,
        })),
    };

    // Inicia com os valores atuais do banco
    let mut nome = current_item.nome.clone();
    let mut descricao = current_item.descricao.clone();
    let mut preco = current_item.preco;
    let mut tipo = current_item.tipo.clone();
    let mut estoque_atual = current_item.estoque_atual;
    let mut estoque_minimo = current_item.estoque_minimo;
    let mut image_url = current_item.image_url.clone();

    while let Some(field) = multipart.next_field().await.unwrap_or(None) {
        match field.name().unwrap_or("") {
            "nome" => nome = field.text().await.unwrap_or(nome.clone()),
            "descricao" => descricao = Some(field.text().await.unwrap_or_default()),
            "preco" => preco = field.text().await.unwrap_or_default().parse().unwrap_or(preco),
            "tipo" => tipo = Some(field.text().await.unwrap_or_default()),
            "estoque_atual" => estoque_atual = field.text().await.unwrap_or_default().parse().unwrap_or(estoque_atual),
            "estoque_minimo" => estoque_minimo = field.text().await.unwrap_or_default().parse().unwrap_or(estoque_minimo),
            // Troca imagem apenas se enviar uma nova
            "image" => if let Some(url) = extract_image(field).await { image_url = Some(url); },
            _ => {}
        }
    }

    let result = sqlx::query_as::<_, ItemResponse>(
        "UPDATE items
        SET nome = $1, descricao = $2, preco = $3, image_url = $4, tipo = $5, estoque_atual = $6, estoque_minimo = $7, updated_at = NOW()
        WHERE id = $8
        RETURNING id, nome, descricao, preco, image_url, tipo, estoque_atual, estoque_minimo, updated_at, updated_by"
    )
    .bind(&nome)
    .bind(&descricao)
    .bind(preco)
    .bind(&image_url)
    .bind(&tipo)
    .bind(&estoque_atual)
    .bind(&estoque_minimo)
    .bind(&id)
    .fetch_one(&pool)
    .await;

    match result {
        Ok(item) => (StatusCode::OK, Json(ApiResponse { 
            message: "Item updated successfully".to_string(), data: Some(item),
        })),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse {
            message: format!("Failed to update item: {}", e),
            data: None
        }))
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