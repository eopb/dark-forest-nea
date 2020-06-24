use crate::updates;
use seed::browser::url::Url;
use seed::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Route(glue::Route);

impl Into<glue::Route> for Route {
    fn into(self) -> glue::Route {
        self.0
    }
}

impl From<glue::Route> for Route {
    fn from(route: glue::Route) -> Self {
        Self(route)
    }
}

impl Default for Route {
    fn default() -> Self {
        Self(glue::Route::default())
    }
}

impl From<Url> for Route {
    fn from(url: Url) -> Self {
        match &url
            .path()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => Self(glue::Route::Index),
            ["explore"] => Self(glue::Route::Explore),
            ["sign-in"] => Self(glue::Route::SignIn),
            ["create-account"] => Self(glue::Route::CreateAccount),
            ["new-project"] => Self(glue::Route::NewProject),
            _ => Self(glue::Route::NotFound),
        }
    }
}

impl Route {
    pub fn update(url: Url) -> Option<updates::Msg> {
        Some(updates::Msg::ChangeRoute(url.into()))
    }
    pub fn request_required_data(self, orders: &mut impl Orders<updates::Msg>) {
        if glue::Route::Index == self.into() {
            orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
        };
    }
}
