use chrono::NaiveDateTime;
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use validator::Validate;


#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: Option<String>,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub salt: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Deserialize, Validate)]
pub struct NewUser {
    #[validate(length(min=5))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 14))]
    pub password: String,
    #[validate(must_match="password")]
    pub password2: String
}
