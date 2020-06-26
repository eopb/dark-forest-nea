use {
    async_trait::async_trait,
    seed::{browser::fetch::FetchError, prelude::*},
    serde::{Deserialize, Serialize},
    web_sys::RequestCredentials::SameOrigin,
};
#[async_trait(?Send)]
pub trait Endpoint: 'static + glue::Endpoint + Serialize + for<'a> Deserialize<'a> {
    async fn fetch() -> Result<Self, FetchError> {
        Request::new(Self::PATH)
            // TODO Maybe this is default so lets try removing it when we need it.
            .credentials(SameOrigin)
            .fetch()
            .await?
            .json()
            .await
    }
}

impl Endpoint for glue::Hello {}
impl Endpoint for glue::SignedIn {}
