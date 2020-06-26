use {
    async_trait::async_trait,
    glue::Endpoint as _,
    seed::{browser::fetch::FetchError, prelude::*},
    web_sys::RequestCredentials::SameOrigin,
};
#[async_trait(?Send)]
pub trait Endpoint: glue::Endpoint {
    async fn fetch() -> Result<Self, FetchError>;
}

#[async_trait(?Send)]
impl Endpoint for glue::Hello {
    async fn fetch() -> Result<Self, FetchError> {
        Request::new(Self::PATH)
            //TODO Maybe this is default so lets try removing it when we need it.
            .credentials(SameOrigin)
            .fetch()
            .await?
            .json()
            .await
    }
}
