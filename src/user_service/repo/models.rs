use sqlx::{
    FromRow,
    types::{
        Uuid,
        chrono,
    }
};

#[derive(FromRow, Debug)]
pub struct User {
    pub id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub username: String,
    pub password: String,
}