use anyhow::Context;
use axum::{
    extract::{FromRef, State}, http::{header::SET_COOKIE, HeaderMap, StatusCode}, response::{IntoResponse, Redirect}, routing::{get, post}, Json, Router
};
use oauth2::{basic::BasicClient, CsrfToken, EndpointNotSet, EndpointSet, Scope};
use async_session::{MemoryStore, Session, SessionStore};

use crate::models::app_error::AppError;
use crate::AppState;

static COOKIE_NAME: &str = "SESSION";
static CSRF_TOKEN: &str = "csrf_token";

pub async fn get_userinfo() -> impl IntoResponse {

}

pub async fn spotify_auth(
    State(client): State<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>>,
    State(store): State<MemoryStore>,
) -> Result<impl IntoResponse, AppError> {
    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("streaming".to_string()))
        .url();
    let mut session = Session::new();
    session
        .insert(CSRF_TOKEN, &csrf_token)
        .context("failed ot insert CSRF token into session");

    let cookie = store
        .store_session(session)
        .await
        .context("failed to store CSRF token session");

    let cookie = format!("{COOKIE_NAME}={cookie:?}; SameSite=Lax; HttpOnly; Secure; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );
    
    Ok((headers, Redirect::to(auth_url.as_ref())))
}

pub fn get_routes(state: AppState) -> Router {

    Router::new()
        .route("/auth/spotify/userinfo", get(get_userinfo))
        .route("/auth/spotify", get(spotify_auth))
        .with_state(state)
}
