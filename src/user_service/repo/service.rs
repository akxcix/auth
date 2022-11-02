use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing::{info, error};
use uuid::Uuid;
use crate::user_service::repo::models;

#[derive(Clone)]
pub struct RepoService {
    pool: Pool<Postgres>
}

pub async fn new(url: String, max_connections: u32) -> Result<RepoService, sqlx::Error> {
    let pool = match PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&url)
        .await {
            Ok(pool) => {
                info!("Successfully connected to database!");
                pool
            },
            Err(err) => {
                error!("Unable to connect to database: {}", err);
                return Err(err);
            }
        };

    let repo_service = RepoService{
        pool: pool
    };

    Ok(repo_service)
}

impl RepoService {
    pub async fn fetch_user(self: &Self, username: String) -> Result<Option<models::User>, sqlx::Error> {
        let query = r#"
        SELECT 
            * 
        FROM
            users
        WHERE
            users.username = $1
        "#;

        sqlx::query_as::<_, models::User>(query)
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn add_user(self: &Self, id: Uuid, username: String, password: String) -> Result<models::User, sqlx::Error> {
        let timestamp = chrono::Utc::now();
        
        let query = r#"
        INSERT INTO 
            users(id, created_at, updated_at, username, password)
        VALUES 
            ($1, $2, $3, $4, $5)
        RETURNING
            id, created_at, updated_at, username, password
        "#;
    
        sqlx::query_as::<_, models::User>(query)
        .bind(id)
        .bind(timestamp)
        .bind(timestamp)
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await
    }

    pub async fn update_user_password(
        self: &Self, 
        username: String, 
        password: String
    ) -> Result<models::User, sqlx::Error> {
        let timestamp = chrono::Utc::now();
        
        let query = r#"
        UPDATE
            users
        SET
            password = $1
            updated_at = $2
        WHERE
            username = $3
        RETURNING
            id, created_at, updated_at, username, password
        "#;
    
        sqlx::query_as::<_, models::User>(query)
        .bind(password)
        .bind(timestamp)
        .bind(username)
        .fetch_one(&self.pool)
        .await
    }

}