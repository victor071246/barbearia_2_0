mod models;
mod handlers;
mod response;
mod db;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let pool = db::create_pool().await.expect("Falha to connect to database");
    println!("Connected to PostgreSQL");
}
