#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("requested entity not found")]
    NotFound,

    #[error("an error occurred with the database")]
    Database(sqlx::Error),

    #[error("an error occurred with the database")]
    Argon2(argon2::password_hash::Error),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        Self::Database(err)
    }
}

impl From<argon2::password_hash::Error> for ServiceError {
    fn from(err: argon2::password_hash::Error) -> Self {
        Self::Argon2(err)
    }
}
