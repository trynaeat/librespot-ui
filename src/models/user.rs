use { serde::Deserialize, serde::Serialize };
use anyhow::Context;
use async_session::{MemoryStore, SessionStore};
use axum::{ extract::{FromRef, FromRequestParts}, http::request::Parts, http::header::COOKIE, response::{IntoResponse, Redirect, Response} };
use cookie::Cookie;

static COOKIE_NAME: &str = "SESSION";

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    display_name: String,
    uri: String
}

pub struct AuthRedirect;

impl IntoResponse for AuthRedirect {
    fn into_response(self) -> Response {
        Redirect::temporary("/auth/spotify").into_response()
    }
}

impl<S> FromRequestParts<S> for User
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    // If anything goes wrong or no session is found, redirect to the auth page
    type Rejection = AuthRedirect;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookie_header = parts
            .headers
            .get(COOKIE)
            .ok_or(AuthRedirect);
        let cookie_header = match cookie_header {
            Ok(header) => header.to_str().map_err(|_| AuthRedirect),
            Err(_) => return Err(AuthRedirect),
        };
        let session_cookie = Cookie::split_parse(cookie_header?)
            .find(|cookie| cookie.clone().unwrap().name() == COOKIE_NAME );
        let session_cookie = match session_cookie {
            Some(cookie) => cookie.map_err(|_| AuthRedirect),
            None => return Err(AuthRedirect),
        };

        let session = store
            .load_session(session_cookie?.to_string())
            .await
            .unwrap()
            .ok_or(AuthRedirect)?;

        let user = session.get::<User>("user").ok_or(AuthRedirect)?;

        Ok(user)
    }
}