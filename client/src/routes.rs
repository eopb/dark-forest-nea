use crate::updates;

use seed::{app::subs::UrlChanged, browser::url::Url, prelude::*};

use std::convert::{TryFrom, TryInto};

#[derive(Default, Clone, PartialEq, Eq)]
pub struct Route(pub Option<glue::Route>);

impl From<glue::Route> for Route {
    fn from(route: glue::Route) -> Self {
        Self(Some(route))
    }
}

impl TryFrom<&Url> for Route {
    type Error = ApiRoute;
    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        let qs = &url.search().to_string();
        match &url
            .path()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => Ok(Some(glue::Route::Index.into())),
            ["explore"] => Ok(Some(glue::Route::Explore.into())),
            ["sign-in"] => Ok(Some(glue::Route::SignIn(glue::qs::get_enum(qs)).into())),
            ["create-account"] => Ok(Some(
                glue::Route::CreateAccount(glue::qs::get_enum(qs)).into(),
            )),
            ["new-project"] => Ok(Some(glue::Route::NewProject.into())),
            ["api", ..] => Err(ApiRoute),
            _ => Ok(None),
        }
        .map(Self)
    }
}

pub struct ApiRoute;

impl Route {
    pub fn update(url: UrlChanged) -> Option<updates::Msg> {
        if let Ok(url) = (&url.0).try_into() {
            Some(updates::Msg::ChangeRoute(url))
        } else {
            url.0.go_and_load();
            None
        }
    }
    pub fn request_required_data(self, orders: &mut impl Orders<updates::Msg>) {
        orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::SignedIn));
        if let Some(glue::Route::Index) = self.0 {
            orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
        };
    }
}
