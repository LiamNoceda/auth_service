use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize,};

// Structs for register request
#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

// Structs for register response
#[derive(Serialize)]
pub struct RegisterResponse {
    pub message: String,
    pub username: String,
}

// Handler for users registration
pub async fn register_handler(State(pool): State<PgPool>, Json(payload): Json<RegisterRequest>,) -> Result<(StatusCode, Json<RegisterResponse>), StatusCode> {
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
