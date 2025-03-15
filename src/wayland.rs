use anyhow::{Context, Result};
use std::env;
use wayland_client::Display;

#[allow(dead_code)]
pub struct WaylandHandler {
    _display: Display,
}

impl WaylandHandler {
    #[allow(dead_code)]
    pub fn new() -> Result<Self> {
        let display = Display::connect_to_env().context("Falha ao conectar ao servidor Wayland")?;

        Ok(Self { _display: display })
    }

    pub fn is_wayland_session() -> bool {
        env::var("WAYLAND_DISPLAY").is_ok()
    }

    #[allow(dead_code)]
    pub fn get_display_name(&self) -> String {
        env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| "wayland-0".to_string())
    }
}

pub fn detect_session_type() -> SessionType {
    if WaylandHandler::is_wayland_session() {
        SessionType::Wayland
    } else if env::var("DISPLAY").is_ok() {
        SessionType::X11
    } else {
        SessionType::Unknown
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionType {
    X11,
    Wayland,
    Unknown,
}

impl SessionType {
    pub fn as_str(&self) -> &str {
        match self {
            SessionType::X11 => "X11",
            SessionType::Wayland => "Wayland",
            SessionType::Unknown => "Unknown",
        }
    }
}
