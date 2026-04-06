use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

use crate::users::domain::{CreateUserCommand, User, UserRepository};

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn is_exists(&self, email: &str) -> bool {
        let result = sqlx::query!(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as "exists!"
            "#,
            email
        )
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(record) => record.exists,
            Err(_) => false,
        }
    }

    async fn create(&self, cmd: CreateUserCommand) -> Result<User, String> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, first_name, last_name, username, email, password, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW())
            RETURNING id, first_name, last_name, username, email, password, created_at as "created_at: _", updated_at as "updated_at: _"
            "#,
            Uuid::new_v4(),
            cmd.first_name,
            cmd.last_name,
            cmd.username,
            cmd.email,
            cmd.password
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn get_all(&self) -> Result<Vec<User>, String> {
        let users = sqlx::query_as!(User, r#"SELECT * FROM users"#)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(users)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let user = sqlx::query_as!(User, r#"SELECT * FROM users WHERE email = $1"#, email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }
}
