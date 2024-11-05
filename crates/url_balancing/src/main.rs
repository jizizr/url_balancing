mod dao;
mod error;
mod handler;
mod jwt;
mod middleware;
mod oauth;
mod routers;
mod state;
mod token;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = routers::init_router().await;
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse().unwrap()));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
