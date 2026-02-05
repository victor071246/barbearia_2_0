use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(sqlx::FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub created_at: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct CreateItemRequest {}