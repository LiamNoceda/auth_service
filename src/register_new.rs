use axum::{
    response::{IntoResponse, Response},
    extract::State,
    http::StatusCode,
    Json,
};
use argon2::{
    password_hash::{PasswordHasher, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize,};
use sqlx::PgPool;
use std::sync::Arc;
use validator::Validator;

// Data Base configuration struct
pub struct AppConfig {
    pub db: PgPool,
}

// Struct for register request
#[derive(Deserialize, Validator)]
pub struct RegisterRequest {
    #[validate(length(min = 2, max = 55, message = "Username must be between 2 and 55 characters"))]
    pub username: String,

    #[validate(length(min = 8, max = 130, message = "The Password must be between 8 and 130 characters"))]
    pub password: String,
}

// Struct for auth response
#[derive(Serialize)]
pub struct AuthResponse {
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub enum AppError {
    ValidationError(String),
    UserAlreadyExists,
    DatabaseError(sqlx::Error),
    InternalServerError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_messge) = match self {
            AppError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::UserAlreadyExists => (StatusCode::CONFLICT, "Username is already taken".to_string()),
            AppError::DatabaseError(e) => {
                eprintln!("Database error occurred: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database server error".to_string())
            }
            AppError::InternalServerError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error".to_string()),
        };

        (status, Json(ErrorResponse { error, error_messge})).into_response()
    }
}

pub async fn register_handler(State(ctx): State<Arc>, Json(payload): Json<RegisterRequest>,) -> Result<impl IntoResponse, AppError> {
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
