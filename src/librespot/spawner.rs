use core::fmt;
use std::{error, f32::consts::E, io::Error, ops::DerefMut, process::{Child, Command, ExitStatus}};

use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use crate::models::user::User;

#[derive(Clone)]
pub struct LibrespotConfig {
    pub backend: Option<String>,
    pub name: Option<String>,
    pub cache_path: String,
}

#[derive(Serialize, Deserialize)]
pub enum LibrespotStatus {
    Running,
    Stopped,
}

impl LibrespotStatus {
    fn as_str(&self) -> &'static str {
        match self {
            LibrespotStatus::Running => "Running",
            LibrespotStatus::Stopped => "Stopped",
        }
    }
}

pub struct LibrespotInst {
    process: Option<Child>,
    status: LibrespotStatus,
    config: Option<LibrespotConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct LibrespotInfo{
    status: LibrespotStatus,
    pid: u32,
    stopped_status: Option<i32>,
}

#[derive(Debug)]
pub struct InstExistsError {}
#[derive(Debug)]
pub struct InstDoesNotExistError {}
#[derive(Debug)]
pub enum SpawnError {
    InstExistsError,
    InstDoesNotExistError,
    Error(Error),
}

impl From<Error> for SpawnError {
    fn from(err: Error) -> SpawnError {
        SpawnError::Error(err)
    }
}

impl fmt::Display for SpawnError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SpawnError::Error(..) =>
                write!(f, "Encountered IO Error"),
            SpawnError::InstExistsError =>
                write!(f, "Librespot is already running."),
            SpawnError::InstDoesNotExistError =>
                write!(f, "Librespot is not running."),
        }
    }
}

impl error::Error for SpawnError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            SpawnError::Error(ref e) => Some(e),
            SpawnError::InstExistsError => None,
            SpawnError::InstDoesNotExistError => None,
        }
    }
}

impl IntoResponse for SpawnError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            SpawnError::Error(..) => "Encountered IO Error",
            SpawnError::InstExistsError => "Librespot is already running",
            SpawnError::InstDoesNotExistError => "Librespot is not running",
        };
        match self {
            SpawnError::Error(..) => (StatusCode::INTERNAL_SERVER_ERROR, body).into_response(),
            SpawnError::InstExistsError => (StatusCode::BAD_REQUEST, body).into_response(),
            SpawnError::InstDoesNotExistError => (StatusCode::BAD_REQUEST, body).into_response(),
        }
    }
}

impl LibrespotInst {
    pub fn new(config: Option<LibrespotConfig>) -> Self {
        Self { process: None, status: LibrespotStatus::Stopped, config }
    }

    // If provided a user, spawn with their access token
    // Otherwise use what's in the cache
    pub fn spawn_librespot(&mut self, user: Option<&User>) -> Result<(), SpawnError>{
        let config = self.config.clone().unwrap_or(LibrespotConfig {backend: None, name: None, cache_path: "/tmp/librespot".to_string()});
        // Check if librespot process has died on its own
        let info = self.get_status();
        if matches!(info?.status, LibrespotStatus::Stopped) {
            self.status = LibrespotStatus::Stopped;
        }

        if self.process.is_none() || matches!(self.status, LibrespotStatus::Stopped) {
            let mut cmd = Command::new("librespot");
            cmd
                .arg("--backend")
                .arg(config.backend.unwrap_or("pipe".to_string()))
                .arg("-n")
                .arg(config.name.unwrap_or("librespot-ui".to_string()))
                .arg("--cache")
                .arg(config.cache_path);
            if let Some(u) = user {
                cmd
                    .arg("--access-token")
                    .arg(u.token.clone().unwrap_or("".to_string()));
            };
            let result = cmd.spawn();
            self.process = Some(result?);
            self.status = LibrespotStatus::Running;
            return Ok(())
        } else {
            Err(SpawnError::InstExistsError {})
        }
    }

    pub fn kill_librespot(&mut self) -> Result<(), SpawnError> {
        let result = match &mut self.process {
            Some(ref mut p) => Ok(p.kill()?),
            None => Err(SpawnError::InstDoesNotExistError {}),
        };
        self.status = LibrespotStatus::Stopped;
        return result;
    }

    pub fn get_status(&mut self) -> Result<LibrespotInfo, SpawnError> {
        if self.process.is_none() {
            Ok(LibrespotInfo{status: LibrespotStatus::Stopped, pid: 0, stopped_status: None})
        } else {
            let p: &mut Child = &mut self.process.as_mut().unwrap();
            let pid = p.id();
            let stopped_status = p.try_wait()?;
            match stopped_status {
                Some(p) => Ok(LibrespotInfo{status: LibrespotStatus::Stopped, pid, stopped_status: p.code() }),
                None => Ok(LibrespotInfo { status: LibrespotStatus::Running, pid, stopped_status: None })
            }
        }
    }
}