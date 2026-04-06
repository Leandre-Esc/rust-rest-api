use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::users::domain::{CreateUserCommand, UpdateUserCommand, User, UserRepository};
use crate::users::queries;

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[derive(FromRow)]
pub struct ExistsResult {
    exists: Option<bool>,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn is_exists(&self, email: &str) -> bool {
        let result = sqlx::query_as::<_, ExistsResult>(queries::IS_EXISTS)
            .bind(email)
            .fetch_one(&self.pool)
            .await;

        match result {
            Ok(record) => record.exists.unwrap_or(false),
            Err(_) => false,
        }
    }

    async fn get_all(&self) -> Result<Vec<User>, String> {
        let users = sqlx::query_as::<_, User>(queries::GET_ALL)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(users)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, String> {
        let user = sqlx::query_as::<_, User>(queries::GET_BY_ID)
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn get_by_email(&self, email: &str) -> Result<Option<User>, String> {
        let user = sqlx::query_as::<_, User>(queries::GET_BY_EMAIL)
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn create(&self, cmd: CreateUserCommand) -> Result<User, String> {
        let user = sqlx::query_as::<_, User>(queries::CREATE)
            .bind(Uuid::new_v4())
            .bind(cmd.first_name)
            .bind(cmd.last_name)
            .bind(cmd.username)
            .bind(cmd.email)
            .bind(cmd.password)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }

    async fn update(&self, cmd: UpdateUserCommand) -> Result<User, String> {
        let user = sqlx::query_as::<_, User>(queries::UPDATE)
            .bind(cmd.first_name)
            .bind(cmd.last_name)
            .bind(cmd.username)
            .bind(cmd.email)
            .bind(cmd.password)
            .bind(cmd.id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(user)
    }
}
