use crate::user_service;
use crate::api::{
    models::{
        dtos,
        requests,
        responses
    },
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
    Json(payload): Json<requests::create_user::CreateUser>,
    user_service: Arc<user_service::service::UserService>
) -> impl IntoResponse {
    match user_service.create_user(payload.username, payload.password).await {
        Ok(new_user) => {
            let user = dtos::user::User{
                id: new_user.id,
                username: new_user.username
            };

            let data = responses::create_user::CreateUser{
                user: user
            };


            Response::ok(data)
        }
        Err(err) => {
            Response::error(StatusCode::INTERNAL_SERVER_ERROR ,err.to_string())
        }
    }
}


pub async fn root() -> &'static str {
    "like a charm!"
}