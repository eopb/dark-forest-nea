use crate::updates;
use seed::browser::url::Url;
use seed::prelude::*;

#[derive(Clone, PartialEq, Eq)]
pub enum Unknown {
    NotFound,
    Api,
}

impl Default for Unknown {
    fn default() -> Self {
        Self::NotFound
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct Route(Result<glue::Route, Unknown>);

impl From<Route> for Result<glue::Route, Unknown> {
    fn from(t: Route) -> Self {
        t.0
    }
}

impl<'a> From<&'a Route> for &'a Result<glue::Route, Unknown> {
    fn from(t: &'a Route) -> Self {
        &t.0
    }
}

impl From<glue::Route> for Route {
    fn from(route: glue::Route) -> Self {
        Self(Ok(route))
    }
}

impl From<Unknown> for Route {
    fn from(route: Unknown) -> Self {
        Self(Err(route))
    }
}

impl Default for Route {
    fn default() -> Self {
        Unknown::default().into()
    }
}

impl From<Url> for Route {
    fn from(url: Url) -> Self {
        let qs = &url.search().to_string();
        match &url
            .path()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => glue::Route::Index.into(),
            ["explore"] => glue::Route::Explore.into(),
            ["sign-in"] => glue::Route::SignIn(glue::qs::get_enum(qs)).into(),
            ["create-account"] => glue::Route::CreateAccount(glue::qs::get_enum(qs)).into(),
            ["new-project"] => glue::Route::NewProject.into(),
            ["api", ..] => Unknown::Api.into(),
            _ => Unknown::NotFound.into(),
        }
    }
}

impl Route {
    pub fn update(url: Url) -> Option<updates::Msg> {
        Some(updates::Msg::ChangeRoute(url.into()))
    }
    pub fn request_required_data(self, orders: &mut impl Orders<updates::Msg>) {
        orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::SignedIn));
        if let Ok(glue::Route::Index) = self.into() {
            orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
        };
    }
}
