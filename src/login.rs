// Test Sysytem

use axum::{
    extract::State,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validate;


#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(length(min = 2, max = 55, message = "Username must be between 2 and 55 characters"))]
    pub username: String,
    #[validate(length(min = 8, max = 130, message = "The Password must be between 8 and 130 characters"))]
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn login_handler(State(ctx): State<Arc<PgPool>>, Json(payload): Json<LoginRequest>) -> Result<impl IntoResponse, StatusCode> {
    // 1 Валидция данных

    // 2 Сверка данных базы данных

    // 3 отправка токена при успехе

    // 4 При успехе отправить статус код успеха
}