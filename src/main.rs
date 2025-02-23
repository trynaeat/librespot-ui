use std::{env, sync::{LazyLock, Mutex}, error};

use anyhow::{anyhow, Context, Result};
use async_session::MemoryStore;
use axum::{
    extract::FromRef, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router
};
use librespot::spawner::{LibrespotConfig, LibrespotInst};
use oauth2::{basic::BasicClient, AuthUrl, ClientId, ClientSecret, EndpointNotSet, EndpointSet, RedirectUrl, TokenUrl};
use serde::{Deserialize, Serialize};
use clap::Parser;

mod routes;
mod models;
mod librespot;
use models::app_error::AppError;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 3000)]
    port: i32,
    #[arg(short, long, default_value_t = String::from("alsa"))]
    backend: String,
    #[arg(short, long, default_value_t = String::from("librespot-ui"))]
    name: String,
}

#[derive(Clone)]
struct AppState {
    store: MemoryStore,
    oauth_client: BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet>,
}

impl FromRef<AppState> for MemoryStore {
    fn from_ref(state: &AppState) -> Self {
        state.store.clone()
    }
}

impl FromRef<AppState> for BasicClient<EndpointSet, EndpointNotSet, EndpointNotSet, EndpointNotSet, EndpointSet> {
    fn from_ref(state: &AppState) -> Self {
        state.oauth_client.clone()
    }
}

static LIBERESPOT_INST: LazyLock<Mutex<LibrespotInst>> = LazyLock::new(|| {
    Mutex::new(LibrespotInst::new(None))
});

fn oauth_client() -> Result<BasicClient, AppError> {
    // Environment variables (* = required):
    // *"CLIENT_ID"     "REPLACE_ME";
    // *"CLIENT_SECRET" "REPLACE_ME";
    //  "REDIRECT_URL"  "http://127.0.0.1:3000/auth/authorized";
    //  "AUTH_URL"      "https://discord.com/api/oauth2/authorize?response_type=code";
    //  "TOKEN_URL"     "https://discord.com/api/oauth2/token";

    let client_id = env::var("CLIENT_ID").context("Missing CLIENT_ID!")?;
    let client_secret = env::var("CLIENT_SECRET").context("Missing CLIENT_SECRET!")?;

    let client = BasicClient::new(ClientId::new(client_id))
        .set_client_secret(ClientSecret::new(client_secret));
    
    Ok(client)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>>{
    let redirect_url = env::var("REDIRECT_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:3000/auth/authorized".to_string());

    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| {
        "https://accounts.spotify.com/authorize".to_string()
    });

    let token_url = env::var("TOKEN_URL")
        .unwrap_or_else(|_| "https://accounts.spotify.com/api/token".to_string());

    let store = MemoryStore::new();
    let oauth_client = oauth_client().unwrap();
    let oauth_client_init = oauth_client.clone()
        .set_auth_uri(AuthUrl::new(auth_url).context("Failed auth_uri").unwrap())
        .set_token_uri(TokenUrl::new(token_url).context("failed token_uri").unwrap())
        .set_redirect_uri(RedirectUrl::new(redirect_url).context("Failed redirect_uri").unwrap());
    let app_state = AppState {
        store,
        oauth_client: oauth_client_init,
    };

    let args = Args::parse();
    let config = LibrespotConfig {
        backend: Some(args.backend),
        name: Some(args.name),
    };
    let mut libre_inst = LIBERESPOT_INST.lock()?;
    *libre_inst = LibrespotInst::new(Some(config));
    // Release the mutex lock
    drop(libre_inst);

    let app = Router::new()
        .merge(routes::auth::get_routes(app_state.clone()))
        .merge(routes::librespot::get_routes(app_state))
        .fallback(handler_404);

    println!("Listening on port {}", args.port);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    Ok(axum::serve(listener, app).await?)
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "404 NOT FOUND")
}