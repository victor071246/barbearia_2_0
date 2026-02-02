use axum::{Json, extract::Path, http::StatusCode};
use crate::models::user::*;
use crate::response::{ApiResponse};

pub async fn create_user(Json(payload): Json<CreateUserRequest>)