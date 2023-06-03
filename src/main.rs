use axum::{response::IntoResponse, Json};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions};
use std::sync::Arc;
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use tower_http::cors::CorsLayer;
use pets::controller::create_service;
use pets::AppState;

async fn healthcheck() -> impl IntoResponse {
    let response = serde_json::json!({
        "status": "success"
    });
    Json(response)
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new().max_connections(10).connect(&db_url).await{
        Ok(pool) => {
            pool
        }
        Err(_err) => {
            std::process::exit(1);
        }
    };
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);
    let app = create_service(Arc::new(AppState { db: pool.clone() })).layer(cors);
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap()).serve(app.into_make_service()).await.unwrap()
}
