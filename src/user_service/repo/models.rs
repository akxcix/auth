use sqlx;

#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: sqlx::types::Uuid,
    pub username: String,
    pub password: String,
}