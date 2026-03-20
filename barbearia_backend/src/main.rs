mod models;
mod handlers;
mod response;
mod routes;
mod db;
mod middleware;

use tower_http::cors::{CorsLayer, Any};
use axum::{Router, http::{Method, header::{AUTHORIZATION, CONTENT_TYPE, COOKIE}}};
use routes::{user_routes::user_routes, item_routes::item_routes, auth_routes::auth_routes};


use crate::routes::health_routes::health_routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let cors = CorsLayer::new().
        allow_origin("http://localhost:5173".parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, COOKIE])
        .allow_credentials(true);

    let pool = db::create_pool().await.expect("Failed to connect to database");
    let app = Router::new()
    .merge(user_routes(pool.clone()))
    .merge(auth_routes(pool.clone()))
    .merge(item_routes(pool.clone()))
    .merge(health_routes(pool.clone()))
    .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
