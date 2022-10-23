use serde::Serialize;

#[derive(Serialize)]
pub struct VerifyUser {
    pub token: String
}