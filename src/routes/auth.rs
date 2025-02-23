use anyhow::{anyhow, Context};
use axum::{
    extract::{Query, State}, http::{header::SET_COOKIE, HeaderMap, header::COOKIE, StatusCode }, response::{IntoResponse, Redirect}, routing::{get, head, post}, Json, Router
};
use oauth2::{basic::BasicClient, AuthorizationCode, CsrfToken, EndpointNotSet, EndpointSet, Scope, TokenResponse};
use async_session::{MemoryStore, Session, SessionStore};
use serde::Deserialize;
use cookie::Cookie;

use crate::models::{app_error::AppError, user::User};
use crate::AppState;

static COOKIE_NAME: &str = "SESSION";
static CSRF_TOKEN: &str = "csrf_token";

async fn csrf_token_validation_workflow(
    auth_request: &AuthRequest,
    cookie: &Cookie<'_>,
    store: &MemoryStore,
) -> Result<(), AppError> {
    // Extract the cookie from the request
    let cookie = cookie
        .value();

    // Load the session
    let session = match store
        .load_session(cookie.to_string())
        .await
        .context("failed to load session")?
    {
        Some(session) => session,
        None => return Err(anyhow!("Session not found").into()),
    };

    // Extract the CSRF token from the session
    let stored_csrf_token = session
        .get::<CsrfToken>(CSRF_TOKEN)
        .context("CSRF token not found in session")?
        .to_owned();

    // Cleanup the CSRF token session
    store
        .destroy_session(session)
        .await
        .context("Failed to destroy old session")?;

    // Validate CSRF token is the same as the one in the auth request
    if *stored_csrf_token.secret() != auth_request.state {
        return Err(anyhow!("CSRF token mismatch").into());
    }

    Ok(())
}

pub async fn get_userinfo(
    user: User,
) -> Result<impl IntoResponse, AppError> {
    Ok((StatusCode::OK, Json(user)))
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
        .context("failed ot insert CSRF token into session")
        .context("Session failure");

    let cookie = store
        .store_session(session)
        .await
        .context("failed to store CSRF token session");

    let cookie = match cookie? {
        Some(cookie) => cookie,
        None => "".to_string()
    };
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; HttpOnly; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(
        SET_COOKIE,
        cookie.parse().context("failed to parse cookie")?,
    );
    
    Ok((headers, Redirect::to(auth_url.as_ref())))
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AuthRequest {
    code: String,
    state: String,
}

pub async fn login_authorized(
    headers: HeaderMap,
    Query(query): Query<AuthRequest>,
    State(store): State<MemoryStore>,
    State(oauth_client): State<BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>>,
) -> Result<impl IntoResponse, AppError> {
    let cookie_header = headers.get(COOKIE).context("Failed to get Cookie")?.to_str()?;
    let cookies = Cookie::parse(cookie_header)?;
    csrf_token_validation_workflow(&query, &cookies, &store).await?;

    let http_client = reqwest::ClientBuilder::new()
        .build()
        .expect("Client should build");
    let token = oauth_client
        .exchange_code(AuthorizationCode::new(query.code.clone()))
        .request_async(&http_client)
        .await
        .context("Failed to request token from auth server")?;

    let user_data = http_client
        .get("https://api.spotify.com/v1/me")
        .bearer_auth(token.access_token().secret())
        .send()
        .await
        .context("Failed sending UserInfo request")?
        .json::<User>()
        .await
        .context("Failed to deserialize UserInfo");
    let mut session = Session::new();
    let _ = session
        .insert("access_token", token.access_token().secret());
    let _ = session
        .insert("user", &user_data?);
    let cookie = store
        .store_session(session)
        .await
        .context("Failed to store session");
    let cookie = match cookie {
        Ok(c) => c.unwrap(),
        Err(_) => "".to_string(),
    };
    let cookie = format!("{COOKIE_NAME}={cookie}; SameSite=Lax; HttpOnly; Path=/");
    let mut headers = HeaderMap::new();
    headers.insert(SET_COOKIE, cookie.parse().context("Failed to parse cookie")?);
    Ok((headers, Redirect::to("/")))
}

pub async fn logout(
    State(store): State<MemoryStore>,
    headers: HeaderMap,
) {
    
}

pub fn get_routes(state: AppState) -> Router {

    Router::new()
        .route("/api/auth/spotify/userinfo", get(get_userinfo))
        .route("/api/auth/spotify", get(spotify_auth))
        .route("/api/auth/authorized", get(login_authorized))
        .with_state(state)
}
