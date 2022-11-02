use crate::user_service::repo::{
    models,
    service
};
use crate::user_service::error::ServiceError;
use uuid::Uuid;
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

#[derive(Clone)]
pub struct UserService<'a> {
    repo_service: service::RepoService,
    argon2: Argon2<'a>,
}
impl<'a> UserService<'a> {
    pub async fn new(dsn: String) -> Result<Box<UserService<'a>>, sqlx::Error> {
        let repo_service = service::new(dsn, 5).await?;
        let argon2 = Argon2::default();

        let service = UserService{
            repo_service: repo_service,
            argon2: argon2,
        };

        let boxed_service = Box::new(service);
        Ok(boxed_service)
    }

    pub async fn create_user(
        self: &Self,
        username: String,
        password: String
    ) -> Result<models::User, ServiceError> {
        let salt = SaltString::generate(&mut OsRng);
        let hashed_password = self.argon2.hash_password(password.as_bytes(), &salt).unwrap().to_string();
        let id = Uuid::new_v4();

        let result = self.repo_service.add_user(id, username, hashed_password).await?;

        Ok(result)
    }

    pub async fn verify_user(
        self: &Self,
        username: String,
        password: String,
    ) -> Result<Option<models::User>, ServiceError> {
        let user_opt = self.repo_service.fetch_user(username).await?;
        match user_opt  {
            Some(user) => {
                let parsed_hash = PasswordHash::new(&user.password)?;
                match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(_) => Ok(Some(user)),
                    Err(_) => Err(ServiceError::NotFound)
                }
            },
            None => Err(ServiceError::NotFound)
        }
    }

    pub async fn update_password(
        self: &Self,
        username: String,
        password: String,
        new_password: String,
    ) -> Result<Option<models::User>, ServiceError> {
        let user_opt = self.repo_service.fetch_user(username.clone()).await?;
        match user_opt  {
            Some(user) => {
                let parsed_hash = PasswordHash::new(&user.password)?;
                match self.argon2.verify_password(password.as_bytes(), &parsed_hash) {
                    Ok(_) => {
                        let salt = SaltString::generate(&mut OsRng);
                        let hashed_new_password = self.argon2.hash_password(new_password.as_bytes(), &salt).unwrap().to_string();

                        let result = self.repo_service.update_user_password(username, hashed_new_password).await?;

                        Ok(Some(result))
                    },
                    Err(_) => Err(ServiceError::NotFound)
                }
            },
            None => Err(ServiceError::NotFound)
        }
    }
}
