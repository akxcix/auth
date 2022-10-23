use crate::user_service;
use crate::api::{
    models,
};

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

            let response = models::responses::server_response::Response::ok(data);

            (StatusCode::CREATED, Json(response))
        }
        Err(err) => {
            let response = models::responses::server_response::Response::server_error(err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}