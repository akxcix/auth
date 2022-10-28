use crate::user_service;
use crate::api;

use axum::{
    routing::{get, post},
    Router
};

pub async fn get_router() -> Result<Router, sqlx::Error> {
    let service = user_service::service::UserService::new().await?;
    let router = Router::new()
        .route("/users/create", post({
            let user_service = service.clone();
            move |body| api::handler::create_user(body, user_service)
        }))
        .route("/users/login", post({
            let user_service = service.clone();
            move |body| api::handler::verify_user(body, user_service)
        }))
        .route("/areyouok", get(api::handler::healthcheck));
    
    Ok(router)
}