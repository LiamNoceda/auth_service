// Test Sysytem

use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::{
    Deserialize,
    Serialize,
};
use sqlx::PgPool;
use std::sync::Arc;


#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub message: String,
    pub token: String,
}

pub fn login_handler(State(ctx): State<Arc<AppConfig>>, Json(paylod): Json<LoginRequest>) -> Result<(StatusCode, Json<LoginResponse>), StatusCode> {
    // 1 Валидция данных

    // 2 Сверка данных базы данных

    // 3 отправка токена при успехе

    // 4 При успехе отправить статус код успеха
}