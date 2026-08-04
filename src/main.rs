use axum::{
    routing::post,
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod register_new;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("Failed to read DATABASE_URL from environment");
    let pool = PgPoolOptions::new()
        .max_connections(5)
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
    
    let addr: SocketAddr = "127.0.0.1:8081".parse().unwrap();
    println!("Server run on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
