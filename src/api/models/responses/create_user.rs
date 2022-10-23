use serde::Serialize;
use crate::api::models::dtos::user::User;

#[derive(Serialize)]
pub struct CreateUser {
    pub user: User
}