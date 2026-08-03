use axum::{
    routing::post,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod register_new;

#[tokio::main]
async fn main() {
    let DATABASE_URL = "YOUR_URL_DATABASE";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(DATABASE_URL)
        .await
        .expect("Failed connect in Data Base");

    let app = Router::new()
        .route("/auth/register", post(register_new::register_handler))
        .with_state(pool);

    let addr = SocketAddr::from(([127,0,0,1], 8081));
    println!("Server run on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
