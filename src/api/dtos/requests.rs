use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdatePassword {
    pub username: String,
    pub password: String,
    pub new_password: String,
}

#[derive(Deserialize)]
pub struct VerifyUser {
    pub username: String,
    pub password: String,
}
