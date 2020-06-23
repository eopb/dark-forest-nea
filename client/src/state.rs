pub mod server;

pub use server::Server;

use crate::routes::Route;

// `Model` describes our app state.
#[derive(Default)]
pub struct Model {
    pub theme: Theme,
    pub route: Route,
    pub server: Server,
}

pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

impl Theme {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }
    pub fn background(&self) -> &str {
        match self {
            Self::Dark => "#090909",
            Self::Light => "#ddd",
        }
    }
    pub fn text(&self) -> &str {
        match self {
            Self::Dark => "#00FF00",
            Self::Light => "#003300",
        }
    }
    pub fn toggle_buttons_color(&self) -> &str {
        match self {
            Self::Dark => "#FFF",
            Self::Light => "#000",
        }
    }
}
