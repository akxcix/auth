mod api;
mod user_service;
use std::net::SocketAddr;
use axum::Server;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();
    
    info!("Starting server...");
    let router = api::router::get_router().await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    
    info!("listening on {}", addr);
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}