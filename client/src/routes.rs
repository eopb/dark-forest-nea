//! User facing routes.

use crate::updates;

use seed::{app::subs::UrlChanged, browser::url::Url, log, prelude::*};

use std::convert::{TryFrom, TryInto};

/// Contains `None` when a route is not found.
#[derive(Default, Clone, PartialEq, Eq, Debug)]
pub struct Route(pub Option<shared::Route>);

impl From<shared::Route> for Route {
    fn from(route: shared::Route) -> Self {
        Self(Some(route))
    }
}

impl TryFrom<&Url> for Route {
    type Error = ApiRoute;
    fn try_from(url: &Url) -> Result<Self, Self::Error> {
        match &url
            .path()
            .iter()
            .map(AsRef::as_ref)
            .collect::<Vec<&str>>()
            .as_slice()
        {
            [] => Ok(Some(shared::Route::Index)),
            ["explore"] => Ok(Some(shared::Route::Explore)),
            ["sign-in"] => Ok(Some(shared::Route::SignIn)),
            ["create-account"] => Ok(Some(shared::Route::CreateAccount)),
            ["new-project"] => Ok(Some(shared::Route::NewProject)),
            ["api", ..] => Err(ApiRoute),
            _ => Ok(None),
        }
        .map(Self)
    }
}

/// Error when the client can not handle an endpoint.
pub struct ApiRoute;

impl Route {
    #[allow(clippy::needless_pass_by_value)] // Update function does not register otherwise.
    /// Parse URL and inform with a message to `update`.
    pub fn update(url: UrlChanged) -> Option<updates::Msg> {
        if let Ok(url) = (&url.0).try_into() {
            Some(updates::Msg::ChangeRoute(url))
        } else {
            url.0.go_and_load();
            None
        }
    }
    /// Request data required by an endpoint to be attached to the model.
    pub fn request_required_data(&self, orders: &mut impl Orders<updates::Msg>) {
        log!("hi bro");
        orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::SignedIn));
        if let Some(shared::Route::Index) = self.0 {
            orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
        };
    }
}
