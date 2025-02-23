use axum::{response::IntoResponse, Router, routing::get, http::StatusCode, Json};

use crate::{librespot::spawner::SpawnError, models::{app_error::AppError, user::User}, AppState, LIBERESPOT_INST};

pub async fn start_librespot (_user: User) -> Result<impl IntoResponse, SpawnError> {
    return match LIBERESPOT_INST.lock().unwrap().spawn_librespot(Some(&_user)) {
        Ok(_) => Ok((StatusCode::OK, "Started.")),
        Err(e) => Err(e),
    };
}

pub async fn stop_librespot (_user: User) -> Result<impl IntoResponse, AppError> {
    return match LIBERESPOT_INST.lock().unwrap().kill_librespot() {
        Ok(_) => Ok((StatusCode::OK, "Stopped.")),
        Err(_) => Ok((StatusCode::BAD_REQUEST, "Already Stopped."))
    };
}

pub async fn get_status (_user: User) -> Result<impl IntoResponse, SpawnError> {
    match LIBERESPOT_INST.lock().unwrap().get_status() {
        Ok(info) => Ok((StatusCode::OK, Json(info))),
        Err(e) => Err(e),
    }
}

pub fn get_routes(state: AppState) -> Router {
    Router::new()
        .route("/api/librespot/start", get(start_librespot))
        .route("/api/librespot/stop", get(stop_librespot))
        .route("/api/librespot/status", get(get_status))
        .with_state(state)
}