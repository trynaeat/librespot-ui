use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
    response::IntoResponse,
};

pub async fn get_userinfo() -> impl IntoResponse {

}

pub fn get_routes() -> Router {
    Router::new()
        .route("/userinfo", get(get_userinfo))
}
