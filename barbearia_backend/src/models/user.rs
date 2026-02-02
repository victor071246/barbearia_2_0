use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub new_username: Option<String>,
    pub user_new_email: Option<String>,
    pub user_new_password: Option<String>,
}

#[derive(Serialize)]
pub struct UserResponse{
    pub id: i32,
    pub username: String,
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>
}

