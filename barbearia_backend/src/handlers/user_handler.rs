use axum::{Json, extract::Path, http::StatusCode};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::response::ApiResponse;

//CREATE
pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    todo!()
}

//READ (um por ID)
pub async fn get_user_by_id(Path(id): Path<i32>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    todo!()
}

//UPDATE
pub async fn update_user(Path(id): Path<i32>, Json(payload): Json<UpdateUserRequest>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    todo!()
}