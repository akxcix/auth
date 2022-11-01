#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform that action")]
    Forbidden,

    #[error("requested entity not found")]
    NotFound,

    #[error("an error occurred with the database")]
    Sqlx(sqlx::Error),
}

impl From<sqlx::Error> for ServiceError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
