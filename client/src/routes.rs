use crate::updates;
use seed::browser::url::Url;
use seed::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Route {
    Index,
    Explore,
    NotFound,
}

impl Default for Route {
    fn default() -> Self {
        Self::NotFound
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
            [] => Self::Index,
            ["explore"] => Self::Explore,
            _ => Self::NotFound,
        }
    }
}

impl Route {
    pub fn update(url: Url) -> Option<updates::Msg> {
        Some(updates::Msg::ChangeRoute(url.into()))
    }
    pub fn go_to(self) -> &'static str {
        match self {
            Self::Index => "/",
            Self::Explore => "/explore",
            Self::NotFound => panic!("Can not go to 404 route"),
        }
    }
    pub fn request_required_data(self, orders: &mut impl Orders<updates::Msg>) {
        if let Self::Index = self {
            orders.send_msg(updates::Msg::ToFetch(updates::ToFetch::Hello));
        };
    }
}
