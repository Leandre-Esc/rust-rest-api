use uuid::Uuid;
use chrono::{Utc, NaiveDateTime, DateTime};

pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub birthdate: NaiveDateTime,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        first_name: String,
        last_name: String,
        username: String,
        email: String,
        password: String,
        birthdate: NaiveDateTime,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            username,
            email,
            password,
            birthdate,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}