use serde::Deserialize;

#[derive(Deserialize)]
pub struct VerifyUser {
    pub username: String,
    pub password: String,
}