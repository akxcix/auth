use crate::user_service::repo::{
    queries
};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
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
pub struct UserService<'a> {
    repo_service: queries::RepoService,
    argon2: Argon2<'a>,
}

pub async fn new() -> Result<UserService<'static>, sqlx::Error> {
    let connection_string = String::from("postgres://localhost/auth");
    let repo_service = queries::new(connection_string, 5).await?;

    let argon2 = Argon2::default();

    let service = UserService{
        repo_service: repo_service,
        argon2: argon2,
    };

    Ok(service)
}

impl UserService<'_> {
    pub async fn create_user(
        self: &Self,
        username: String,
        password: String
    ) -> Result<queries::User, sqlx::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = self.argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        self.repo_service.add_user(username, hashed_password).await 
    }

    pub async fn verify_user(
        self: &Self,
        username: String,
        password: String,
    ) -> Result<Option<queries::User>, sqlx::Error> {
        let userOpt = self.repo_service.fetch_user(username).await?;
        match userOpt  {
            Some(user) => {
                let parsed_hash = PasswordHash::new(&user.password).unwrap();
                match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(_) => {
                        Ok(Some(user))
                    },
                    Err(_) => {
                        Ok(None)
                    }
                }
            },
            None => {
                Ok(None)
            }
        }
    }
}
