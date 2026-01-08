use axum::{Router, routing::get};
use std::net::SocketAddr;

use backend::db;

async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    // Initialize database pool from DATABASE_URL environment variable
    let _pool = db::init_pool()
        .await
        .expect("Failed to create database pool");

    let app = Router::new().route("/", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
