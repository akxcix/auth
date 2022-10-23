mod api;
mod user_service;

use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let service = Arc::new(user_service::service::new().await?);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/areyouok", get(api::handler::healthcheck))
        .route("/users/create", post({
            let user_service = Arc::clone(&service);
            move |body| api::handler::create_user(body, Arc::clone(&user_service))
        }));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}