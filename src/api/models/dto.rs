use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}