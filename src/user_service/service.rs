use crate::user_service::repo::{
    queries
};

// use axum::{
//     http::StatusCode,
//     response::IntoResponse,
//     Json,
// };
// use serde::{
//     Serialize,
//     Deserialize,
// };

#[derive(Clone)]
pub struct UserService {
    repo_service: queries::RepoService
}

pub async fn new() -> Result<UserService, sqlx::Error> {
    let connection_string = String::from("postgres://localhost/auth");
    let repo_service = queries::new(connection_string, 5).await?;

    let service = UserService{
        repo_service: repo_service
    };

    Ok(service)
}

impl UserService {
    pub async fn create_user(
        self: &Self,
        username: String,
        password: String
    ) -> Result<queries::User, sqlx::Error> {
        self.repo_service.add_user(username, password).await 
    }
}
