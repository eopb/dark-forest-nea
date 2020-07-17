//! User facing routes.

use crate::updates;

use seed::{app::subs::UrlChanged, browser::url::Url, prelude::*};

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

            ["users", user_name, "projects", project_name, "edit"] => {
                Ok(Some(shared::Route::Users {
                    user_name: user_name.to_string(),
                    nest: Some(shared::routes::UserRoute::Projects(Some(
                        shared::routes::Project {
                            project_name: project_name.to_string(),
                            nest: Some(shared::routes::ProjectRoute::Edit),
                        },
                    ))),
                }))
            }
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
        orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::SignedIn));
        if let Some(ref route) = self.0 {
            match route {
                shared::Route::Index => {
                    orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
                }
                shared::Route::Users {
                    user_name,
                    nest:
                        Some(shared::routes::UserRoute::Projects(Some(shared::routes::Project {
                            project_name,
                            nest: Some(shared::routes::ProjectRoute::Edit),
                        }))),
                } => {
                    orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Editor(
                        shared::endpoint::edit::init::ProjectPath {
                            project_name: project_name.to_owned(),
                            user_name: user_name.to_owned(),
                        },
                    )));
                }
                _ => {}
            };
        };
    }
}
