use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    // pub async fn get_user(&self, username: &str) -> Result<Option<User>, sqlx::Error> {
    //     let user = sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
    //         .fetch_optional(&self.pool)
    //         .await?;

    //     Ok(user)
    // }
}
