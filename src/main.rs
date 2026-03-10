// src/main.rs
use axum::{Router, routing::get};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_http::{cors::CorsLayer, services::ServeDir};

// Modules
mod handlers;
mod models;
mod state;

use handlers::{get_movie_videos, get_trending_movies, root, search_content};
use state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let api_key = env::var("TMDB_API_KEY").expect("TMDB_API_KEY must be set in .env");

    let state = AppState {
        tmdb_api_key: api_key,
        client: reqwest::Client::new(),
    };

    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);

    let app = Router::new()
        .route("/", get(root))
        .route("/api/trending", get(get_trending_movies))
        .route("/api/search", get(search_content))
        .route("/api/movie/{id}/videos", get(get_movie_videos))
        .nest_service("/stream", ServeDir::new("assets"))
        .layer(cors)
        .with_state(state);

    //let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!(
        "Server listening on http://{}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
