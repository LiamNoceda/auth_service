use axum::{routing::post, Router,};
use axum::http::{header::{CONTENT_TYPE, AUTHORIZATION}, Method};
use tower_http::cors::{AllowOrigin, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

mod register_new;
use register_new::{register_handler, AppConfig};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed to read DATABASE_URL from environment");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .min_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed connect in Data Base");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed run migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/auth/register", post(register_new::register_handler))
        .with_state(pool)
        .layer(cors);
    
    let addr: SocketAddr = "0.0.0.0:8081".parse().unwrap();
    println!("Server run on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
