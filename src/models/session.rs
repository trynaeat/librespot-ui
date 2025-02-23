use async_session::{MemoryStore, Session, SessionStore};
use axum::{extract::{FromRef, FromRequestParts}, http::{status::StatusCode, header::COOKIE}, response::{IntoResponse, Response}};
use cookie::Cookie;

static COOKIE_NAME: &str = "SESSION";

pub struct SessionError;

impl IntoResponse for SessionError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "Error fetching session").into_response()
    }
}

pub struct MySession(Session);

impl MySession {
    pub fn get_session(&self) -> Session {
        self.0.clone()
    }
}

impl<S> FromRequestParts<S> for MySession
where
    MemoryStore: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = SessionError;
    async  fn from_request_parts(parts: &mut axum::http::request::Parts,state: &S,) -> Result<Self,Self::Rejection> {
        let store = MemoryStore::from_ref(state);

        let cookie_header = parts
            .headers
            .get(COOKIE)
            .ok_or(SessionError);
        let cookie_header = match cookie_header {
            Ok(header) => header.to_str().map_err(|_| SessionError),
            Err(_) => return Err(SessionError),
        };
        let session_cookie = Cookie::split_parse(cookie_header?)
            .find(|cookie| cookie.clone().unwrap().name() == COOKIE_NAME );
        let session_cookie = match session_cookie {
            Some(cookie) => cookie.map_err(|_| SessionError),
            None => return Err(SessionError),
        };

        let session = store
            .load_session(session_cookie?.value().to_string())
            .await
            .unwrap()
            .ok_or(SessionError)?;
        return Ok(MySession(session));
    }
}