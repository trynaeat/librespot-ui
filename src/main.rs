use axum::{
    http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use clap::Parser;

mod routes;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: i32,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let app = Router::new()
        .merge(routes::user_info::get_routes())
        .fallback(handler_404);

    println!("Listening on port {}", args.port);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 NOT FOUND")
}