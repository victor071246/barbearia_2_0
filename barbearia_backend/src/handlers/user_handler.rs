use axum::{Json, extract::Path, http::StatusCode, extract::State};
use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::user::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::response::ApiResponse;

//CREATE
pub async fn create_user(State(pool): State<PgPool>, Json(payload): Json<CreateUserRequest>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {

    //Hash da senha
    let password_hash = match hash(&payload.password, DEFAULT_COST) {
        Ok(hash) => hash,
        Err(_) => {
            let response = ApiResponse {
                message: "Failed to hash password".to_string(),
                data: None,
            };
            return  (StatusCode::INTERNAL_SERVER_ERROR, Json(response));
        }
    };

    // Insert no banco
    let result = sqlx::query_as::<_, (i32, String)>("INSERT INTO users (username, email, password_hash) VALUES ($1, $2, $3) RETURNING id, username")
    .bind(&payload.username)
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&pool).await;

    match result {
        Ok((id, username)) => {
            let user_response = UserResponse { id, username};
            let response = ApiResponse {
                message: "User created".to_string(),
                data: Some(user_response),
            };
            (StatusCode::CREATED, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Failed to create user: {e}"),
                data: None,
            };
            (StatusCode::BAD_REQUEST, Json(response))
        }
    }
}

//READ (um por ID)
pub async fn get_user_by_id(Path(id): Path<i32>, State(pool): State<PgPool>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let result = sqlx::query_as::<_, (i32, String)>("SELECT id, username FROM users WHERE id = $1").bind(id).fetch_one(&pool).await;

    match result {
        Ok((id, username)) => {
            let user_response = UserResponse { id, username};
            let response = ApiResponse {
                message: "User found".to_string(),
                data: Some(user_response),
            };
            (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("User not found: {e}"),
                data: None
            };
            (StatusCode::NOT_FOUND, Json(response))
        }
    }
}

pub async fn get_all_users(State(pool) : State<PgPool>) -> (StatusCode, Json<ApiResponse<Vec<UserResponse>>>){
    let result = sqlx::query_as::<_, (i32, String)>(
        "SELECT id, username FROM users"
    ).fetch_all(&pool)
    .await;
    
    match result {
        Ok(rows) => {
            let users: Vec<UserResponse> = rows
            .into_iter()
            .map(|(id, username)| UserResponse { id, username})
            .collect();

        let response = ApiResponse {
            message: "Users found".to_string(),
            data: Some(users)
        };
        (StatusCode::OK, Json(response))
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Error to find users: {e}"),
                data: None,
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

//UPDATE
pub async fn update_user(Path(id): Path<i32>, State(pool) : State<PgPool>, Json(payload): Json<UpdateUserRequest>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {

    //Busca o usuário atual
    let current_user = sqlx::query_as::<_, (String, String, String)>(
        "SELECT username, email, password_hash FROM users WHERE id = $1"
    ).bind(id)
    .fetch_one(&pool)
    .await;

    let (current_username, current_email, current_password_hash) = match current_user {
        Ok(user) => user,
        Err(e) => {
            let response = ApiResponse {
                message: format!("User not found: {e}"),
                data: None
            };
            return (StatusCode::NOT_FOUND, Json(response));
        }
    };

    // Atualiza os campos (usar o valor se for passado ou mantem o mesmo)
    let new_username = payload.new_username.unwrap_or(current_username);
    let new_email = payload.user_new_email.unwrap_or(current_email);

    let new_password_hash = if let Some(new_password) = payload.user_new_password {
        match hash(&new_password, DEFAULT_COST) {
            Ok(h) => h,
            Err(e) => {
                let response = ApiResponse {
                    message: format!("Error to process password"),
                    data: None
                };
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
            }
        }
        } else {
            current_password_hash
        };

        // Update no banco
        let result = sqlx::query_as::<_, (i32, String)>(
            "UPDATE users SET username = $1, email = $2, password_hash = $3 WHERE id = $4 RETURNING id, username"
        )
        .bind(&new_username)
        .bind(&new_email)
        .bind(&new_password_hash)
        .bind(id)
        .fetch_one(&pool)
        .await;

        match result {
            Ok((id, username)) => {
                let user_response = UserResponse {id, username};
                let response = ApiResponse {
                    message: "User updated successfuly".to_string(),
                    data: Some(user_response)
                };
                (StatusCode::OK, Json(response))
            },
            Err(e) => {
                let response = ApiResponse {
                    message: format!("Error to update user: {e}"),
                    data: None
                };
                (StatusCode::BAD_REQUEST, Json(response))
            }
        }
}

pub async fn delete_user(State(pool): State<PgPool>, Path(id) : Path<i32>) -> (StatusCode, Json<ApiResponse<UserResponse>>) {
    let result = sqlx::query("DELETE FROM users WHERE id = $1")
    .bind(id)
    .execute(&pool)
    .await;

    match result {
        Ok(result) => {
            if result.rows_affected() > 0 {
                let response = ApiResponse {
                    message : "User deteled successfuly".to_string(),
                    data: None
                };
                (StatusCode::OK, Json(response))
            }
            else {
                let response = ApiResponse {
                    message: "User not found".to_string(),
                    data: None
                };
                (StatusCode::NOT_FOUND, Json(response))
            }
        },
        Err(e) => {
            let response = ApiResponse {
                message: format!("Error to delete user: {e}"),
                data: None
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }

}