use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;
use serde_json::json;

pub async fn health(State(pool): State<PgPool>) -> (StatusCode, Json<serde_json::Value>) {
    let ok = sqlx::query("SELECT 1").fetch_one(&pool).await.is_ok();

    if ok {
        (StatusCode::OK, Json(json!({"status": "ok", "db": "connected"})))
    } else {
        (StatusCode::BAD_GATEWAY, Json(json!({"status": "error", "db": "disconnected"})))
    }
}