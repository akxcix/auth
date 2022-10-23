use crate::user_service;
use crate::api::{
    models,
};
use crate::api::models::responses::server_response::Response;

use std::sync::Arc;
use axum::{
    http::StatusCode,
    response::
    Json,
    response::IntoResponse
};

pub async fn create_user(
    Json(payload): Json<models::dto::CreateUserRequest>,
    user_service: Arc<user_service::service::UserService>
) -> impl IntoResponse {
    println!("request recieved: {}, {}", payload.username, payload.password);

    match user_service.create_user(payload.username, payload.password).await {
        Ok(new_user) => {
            let user = models::dto::User{
                id: new_user.id,
                username: new_user.username
            };

            let data = models::responses::create_user_response::CreateUserResponse{
                user: user
            };


            Response::ok(data)
        }
        Err(err) => {
            Response::error(StatusCode::INTERNAL_SERVER_ERROR ,err.to_string())
        }
    }
}