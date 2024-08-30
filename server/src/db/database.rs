use super::{User, UserStatus};
use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct Database {
    pool: PgPool,
}

#[allow(dead_code)]
impl Database {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn check_user_exists(&self, username: &str) -> Result<bool, sqlx::Error> {
        let user = sqlx::query!(
            r#"
            SELECT username FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user.is_some())
    }

    pub async fn create_user(&self, user: &User) -> Result<(), sqlx::Error> {                 
        sqlx::query!(
            r#"
            INSERT INTO users (username, password_hash, created_at, last_online, status, bio)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            user.username,
            user.password_hash,
            user.created_at,
            user.last_online,
            user.status as UserStatus,
            user.bio
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn delete_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            DELETE FROM users   
            WHERE id = $1
            "#,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_user_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_at, last_online, status AS "status!: UserStatus", bio FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, created_at, last_online, status AS "status!: UserStatus", bio FROM users
            ORDER BY username ASC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(users)
    }

    // TODO: Think about checks for updating user and how to prevent misuse
    pub async fn update_user(&self, user: User) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            UPDATE users
            SET username = $1, password_hash = $2, bio = $3
            WHERE id = $4
            "#,
            user.username,
            user.password_hash,
            user.bio,
            user.id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
