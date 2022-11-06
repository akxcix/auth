use sqlx::postgres::PgPoolOptions;
use sqlx::Pool;
use sqlx::Postgres;

pub struct Database {
    connection: Pool<Postgres>
}

impl Database {
    pub async fn new(url: String) -> Result<Database, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await?;

        let database = Database{connection: pool};

        Ok(database)
    }
}