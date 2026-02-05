mod models;
mod handlers;
mod response;
mod routes;
mod db;

use routes::user_routes::user_routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = db::create_pool().await.expect("Falha to connect to database");
    let app = user_routes(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Servidor rodando em http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
