use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use serde::{Deserialize, Serialize,};

// Structs for register request
#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: String,
}

// Structs for register response
#[derive(Serialize)]
struct RegisterResponse {
    message: String,
    username: String,
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
