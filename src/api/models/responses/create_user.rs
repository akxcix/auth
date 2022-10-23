use serde::Serialize;
use crate::api::models::dto::User;

#[derive(Serialize)]
pub struct CreateUser {
    pub user: User
}