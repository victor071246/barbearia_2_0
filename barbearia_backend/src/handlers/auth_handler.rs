use axum::http::response;
use axum::{Json, extract::State, http::{StatusCode, header::{HeaderMap, SET_COOKIE, COOKIE}}};
use sqlx::PgPool;
use bcrypt::verify;
use jsonwebtoken::{decode, encode, Header, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use crate::models::auth::{LoginRequest, LoginResponse};
use crate::models::user::UserResponse;
use crate::response::ApiResponse;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: i32,
    exp: usize,
}

pub async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginRequest>) -> (StatusCode, HeaderMap, Json<ApiResponse<LoginResponse>>) {

    // Busca usuário por username
    let result = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, username, password_hash FROM users WHERE username = $1"
    )
    .bind(&payload.username)
    .fetch_one(&pool)
    .await;

    let (user_id, username, password_hash) = match result {
        Ok(user) => user,
        Err(_) => {
            let response = ApiResponse {
                message: "invalid credentials".to_string(),
                data: None,
            };
            return (StatusCode::UNAUTHORIZED, HeaderMap::new(),Json(response))
        }
    };

    let password_valid = verify(&payload.password, &password_hash).unwrap_or(false);

    if !password_valid {
        let response = ApiResponse {
            message: "Invalid credentials".to_string(),
            data: None
        };
        return (StatusCode::UNAUTHORIZED, HeaderMap::new(), Json(response));
    }

    let expiration = chrono::Utc::now()
    .checked_add_signed(chrono::Duration::days(15))
    .unwrap()
    .timestamp() as usize;

    let claims = Claims {
        sub: user_id,
        exp: expiration
    };

    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env environment");
    let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();

    let cookie = format!("token={token}; HttpOnly; Secure; SameSite=Strict; Path=/; Max-Age=86400");

    let login_response = LoginResponse {
        token,
        user_id,
        username
    };

    let response = ApiResponse {
        message: "Login successful".to_string(),
        data: Some(login_response)
    };

    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().unwrap());

    (StatusCode::OK, headers, Json(response))
}

pub async fn verify_auth(headers: HeaderMap) -> (StatusCode, Json<ApiResponse<bool>>) {

    //Pega o cookie
    let cookie_header = match headers.get(COOKIE) {
        Some(h) => h.to_str().unwrap_or(""),
        None => {
            let response = ApiResponse {
                message: "Not authenticated".to_string(),
                data: Some(false),
            };
            return (StatusCode::UNAUTHORIZED, Json(response))
        }
    };

    // Extrai o token do cookie
    let token = cookie_header.split(';').find_map(|cookie| {
        let mut cookie_processed = cookie.trim().split('=');
        if cookie_processed.next()? == "token" {
            cookie_processed.next()
        } else {
            None
        }
    });

    let token = match token {
        Some(t) => t,
        None => {
            let response = ApiResponse {
                message: "Token not found".to_string(),
                data: Some(false)
            };
            return (StatusCode::UNAUTHORIZED, Json(response))
        }
    };

    // Decodifica e valida o JWT
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env environment");

    let is_valid = decode::<Claims>(token, &DecodingKey::from_secret(secret.as_bytes()), &Validation::default()).is_ok();

    // Retorna true/false
    if is_valid {
        let response = ApiResponse {
            message: "Authenticated".to_string(),
            data: Some(true)
        };
        (StatusCode::OK, Json(response))
    } else {
        let response = ApiResponse {
            message: "Invalid or expired token".to_string(),
            data: Some(false)
        };
        (StatusCode::UNAUTHORIZED, Json(response))
    }
}