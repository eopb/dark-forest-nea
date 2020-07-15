pub mod server;

pub use server::Server;

use crate::{routes::Route, ui};

use seed::browser::web_storage::{LocalStorage, WebStorage};
/// Describes client state.
pub struct Model {
    pub theme: Theme,
    pub route: Route,
    pub server: Server,
    pub route_data: RouteData,
    pub login_token: Option<String>,
}

impl Model {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
            route: Route::default(),
            server: Server::default(),
            route_data: RouteData::default(),
            login_token: LocalStorage::get("Login").ok(),
        }
    }
}
#[derive(Default)]
pub struct RouteData {
    pub sign_in: ui::router::sign_in::Model,
    pub create_account: ui::router::create_account::Model,
    pub new_project: ui::router::new_project::Model,
}

#[derive(Copy, Clone)]
/// Colour theme.
pub enum Theme {
    Dark,
    Light,
}

impl Default for Theme {
    fn default() -> Self {
        Self::Dark
    }
}

// TODO maybe return `seed_style_preview::CssColor` rather than strings.

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
    pub fn error(&self) -> &str {
        match self {
            Self::Dark => "#FF0000",
            Self::Light => "#330000",
        }
    }
    /// Colour of the button used to toggle the theme.
    ///
    /// This is close to the inverse colour of the theme.
    pub fn toggle_button_color(&self) -> &str {
        match self {
            Self::Dark => "#FFF",
            Self::Light => "#000",
        }
    }
}
