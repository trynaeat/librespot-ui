use std::sync::{LazyLock, Mutex};

use axum::{response::IntoResponse, Router, routing::get, http::StatusCode};

use crate::{librespot::spawner::LibrespotInst, models::{app_error::AppError, user::User}, AppState};

static LIBERESPOT_INST: LazyLock<Mutex<LibrespotInst>> = LazyLock::new(|| {
    Mutex::new(LibrespotInst::new())
});

pub async fn start_librespot (_user: User) -> Result<impl IntoResponse, AppError> {
    return match LIBERESPOT_INST.lock().unwrap().spawn_librespot() {
        Ok(_) => Ok((StatusCode::OK, "Started.")),
        Err(_) => Ok((StatusCode::BAD_REQUEST, "Already Running."))
    };
}

pub async fn stop_librespot (_user: User) -> Result<impl IntoResponse, AppError> {
    return match LIBERESPOT_INST.lock().unwrap().kill_librespot() {
        Ok(_) => Ok((StatusCode::OK, "Stopped.")),
        Err(_) => Ok((StatusCode::BAD_REQUEST, "Already Stopped."))
    };
}

pub fn get_routes(state: AppState) -> Router {
    let mut ls_spawner = LibrespotInst::new();

    Router::new()
        .route("/librespot/start", get(start_librespot))
        .route("/librespot/stop", get(stop_librespot))
        .with_state(state)
}