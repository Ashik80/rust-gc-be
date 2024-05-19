use std::env;
use axum::Router;
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

mod models;
mod errors;
mod mapper;

mod modules;
use crate::modules::{
    auth::routes::auth_routes,
    categories::routes::category_routes,
    product_images::routes::product_image_routes,
    products::routes::product_routes
};

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load env variables");

    let database_url = env::var("DATABASE_URL").expect("Database url not present");

    let pool = PgPoolOptions::new().connect(&database_url).await.expect("Failed to connect to database");

    let app_state = AppState { pool };

    let app = Router::new()
        .merge(product_routes())
        .merge(category_routes())
        .merge(product_image_routes())
        .merge(auth_routes())
        .with_state(app_state);

    let listener = TcpListener::bind("127.0.0.1:8080").await.expect("Failed to bind port");

    println!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("Failed to serve app")
}
