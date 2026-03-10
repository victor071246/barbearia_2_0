mod models;
mod handlers;
mod response;
mod routes;
mod db;
mod middleware;

use axum::Router;
use routes::{user_routes::user_routes, item_routes::item_routes, auth_routes::auth_routes};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = db::create_pool().await.expect("Failed to connect to database");
    let app = Router::new()
    .merge(user_routes(pool.clone()))
    .merge(auth_routes(pool.clone()))
    .merge(item_routes(pool.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
