use async_trait::async_trait;
use crate::domain::user::{CreateUserCommand, User, UserRepository};
use crate::infra::postgres::PostgresRepository;
use uuid::Uuid;

#[async_trait]
impl UserRepository for PostgresRepository {
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

    async fn save(&self, cmd: CreateUserCommand) -> Result<User, String> {
        let user = sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, first_name, last_name, username, email, password, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, NOW())
            RETURNING id, first_name, last_name, username, email, password, created_at as "created_at: _"
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
}