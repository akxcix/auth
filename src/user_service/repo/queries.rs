use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


#[derive(sqlx::FromRow, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
}

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
    // pub async fn fetch_users(self: &Self) -> Result<Vec<User>, sqlx::Error> {
    //     let query = "SELECT * FROM users";
        
    //     sqlx::query_as::<_, User>(query)
    //     .fetch_all(&self.pool)
    //     .await
    // }

    pub async fn fetch_user(self: &Self, username: String) -> Result<Option<User>, sqlx::Error> {
        let query = r#"
        SELECT * 
        FROM users
        WHERE users.username = $1
        "#;

        sqlx::query_as::<_, User>(query)
        .bind(username)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn add_user(self: &Self, username: String, password: String) -> Result<User, sqlx::Error> {
        let query = r#"
        INSERT INTO users(username, password)
        VALUES ($1, $2)
        RETURNING id, username, password
        "#;

        sqlx::query_as::<_, User>(query)
        .bind(username)
        .bind(password)
        .fetch_one(&self.pool)
        .await
    }
}