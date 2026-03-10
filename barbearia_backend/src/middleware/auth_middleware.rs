use axum::{
    extract::Request,
    http::{StatusCode, HeaderMap},
    middleware::Next,
    response::Response,
    Json
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::response::ApiResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,  // expiration
    pub role: String // "admin" ou "user"
}

pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {

    // Pega o token do header Authorization
    let cookie_header = headers.get("cookie").and_then(|h| h.to_str().ok());

    let token = match cookie_header {
        Some(cookies) => {
            cookies
                .split(';')
                .find(|c| c.trim().starts_with("token="))
                .and_then(|c| c.split('=').nth(1))
        },
        None => None
    };

    let token = match token {
        Some(t) => t,
        None => {
            let response = ApiResponse {
                message: "Unauthorized - No token found".to_string(),
                data: None,
            };
            return Err((StatusCode::UNAUTHORIZED, Json(response)));
        }
    };

    //Valida JWT
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set in .env environment");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    match token_data {
        Ok(data) => {
            request.extensions_mut().insert(data.claims.sub.clone());
            Ok(next.run(request).await)
        },
        Err(_) => {
            let response = ApiResponse {
                message: "Invalid or expired token".to_string(),
                data: None,
            };
            Err((StatusCode::UNAUTHORIZED, Json(response)))
        }
    }
}