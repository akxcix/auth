use crate::service;

use std::sync::Arc;
use serde::{Serialize, Deserialize};
use axum::{
    http::StatusCode,
    response::
    Json,
    response::IntoResponse
};

pub async fn create_user(
    Json(payload): Json<CreateUser>,
    user_service: Arc<service::UserService>
) -> impl IntoResponse {
    println!("request recieved: {}, {}", payload.username, payload.password);

    match user_service.create_user(payload.username, payload.password).await {
        Ok(new_user) => {
            let user = User{
                id: new_user.id,
                username: new_user.username
            };
            (StatusCode::CREATED, Json(user))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(User { id: 0, username: err.to_string() }))
        }
    }
}

#[derive(Deserialize)]
pub struct CreateUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct User {
    id: i32,
    username: String,
}