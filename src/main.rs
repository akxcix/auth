mod api;
mod user_service;
use std::net::SocketAddr;
use axum::{
    routing::{get, post},
    Router
};

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let service = user_service::service::UserService::new().await?;

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let app = Router::new()
        .route("/users/create", post({
            let user_service = service.clone();
            move |body| api::handler::create_user(body, user_service)
        }))
        .route("/users/login", post({
            let user_service = service.clone();
            move |body| api::handler::verify_user(body, user_service)
        }))
        .route("/areyouok", get(api::handler::healthcheck));
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}