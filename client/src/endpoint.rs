use {
    async_trait::async_trait,
    seed::{browser::fetch::FetchError, prelude::*},
    serde::{Deserialize, Serialize},
};
#[async_trait(?Send)]
pub trait Endpoint: 'static + glue::Endpoint + Serialize + for<'a> Deserialize<'a> {
    async fn fetch() -> Result<Self, FetchError> {
        Request::new(Self::PATH).fetch().await?.json().await
    }
}

impl Endpoint for glue::Hello {}
impl Endpoint for glue::SignedIn {}
