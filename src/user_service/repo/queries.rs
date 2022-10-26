use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use uuid::Uuid;
use crate::user_service::repo::models;

#[derive(Clone)]
pub struct RepoService {
    pool: Pool<Postgres>
}

pub async fn new(url: String, max_connections: u32) -> Result<RepoService, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&url)
        .await?;

    let repo_service = RepoService{
        pool: pool
    };

    Ok(repo_service)
}

impl RepoService {
    pub async fn fetch_user(self: &Self, username: String) -> Result<Option<models::User>, sqlx::Error> {
        let query = r#"
        SELECT * 
        FROM users
        WHERE users.username = $1
        "#;

        sqlx::query_as::<_, models::User>(query)
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn add_user(self: &Self, id: Uuid, username: String, password: String) -> Result<models::User, sqlx::Error> {
        let query = r#"
        INSERT INTO users(id, username, password)
        VALUES ($1, $2, $3)
        RETURNING id, username, password
        "#;

        sqlx::query_as::<_, models::User>(query)
        .bind(id)
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await
    }
}