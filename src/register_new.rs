use axum::{
    response::{IntoResponse, Response},
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use std::sync::Arc;
use serde::{Deserialize, Serialize,};
use validator::Validator;

// Data Base pool (for State)
pub struct AppConfig {
    pub db: PgPool,
}

// Structs for register request
#[derive(Deserialize, Validator)]
pub struct RegisterRequest {
    #[validate(lenght(min = 2, max = 55, message = "Userame must be between 2 to 55 characters long"))]
    pub username: String,

    #[validate(lenght(min = 8, message = "The Password must be at least 8 characters long"))]
    pub password: String,
}

// Structs for register response
#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

pub async fn register_handler(State(ctx): State<Arc<AppConfig>>, Json(payload): Json<RegisterRequest>,) -> Result<impl IntoResponse, AppError> {
    payload.validate().map_err(AppError::ValidationError)?;

    let result = sqlx::query!(
        "INSERT INTO users (username, password) VALUES ($1, $2)", 
        &payload.username, 
        &payload.password
        )
        .execute(&pool)
        .await;

    match result {
        Ok(_) => {
            let response = RegisterResponse {
                message: "User registered in Spatiol".to_string(),
                username: payload.username,
            };
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(_) => {
            Err(StatusCode::CONFLICT)
        }
    }
}
