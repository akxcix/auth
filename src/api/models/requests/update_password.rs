use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub username: String,
    pub password: String,
    pub new_password: String,
}