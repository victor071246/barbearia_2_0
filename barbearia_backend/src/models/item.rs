use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Item{
    pub id: i32,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub image_url: Option<String>,
    pub tipo: Option<String>,
    pub estoque_atual: i32,
    pub estoque_minimo: i32,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<i32>
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub image_url: Option<String>,
    pub tipo: Option<String>,
    pub estoque_atual: i32,
    pub estoque_minimo: i32,
}

#[derive(Deserialize)]
pub struct UpdateItemRequest {
    pub nome: Option<String>,
    pub descricao: Option<String>,
    pub preco: Option<f64>,
    pub image_url: Option<String>,
    pub tipo: Option<String>,
    pub estoque_atual: Option<i32>,
    pub estoque_minimo: Option<i32>,
}

#[derive(Serialize)]
pub struct ItemResponse {
    pub id: i32,
    pub nome: String,
    pub descricao: Option<String>,
    pub preco: f64,
    pub image_url: Option<String>,
    pub tipo: Option<String>,
    pub estoque_atual: i32,
    pub estoque_minimo: i32,
    pub updated_at: Option<DateTime<Utc>>,
    pub updated_by: Option<i32>
}