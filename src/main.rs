mod api;
mod user_service;
use std::net::SocketAddr;
use axum::Server;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let router = api::router::get_router().await?;
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    Ok(())
}