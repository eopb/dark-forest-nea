use async_trait::async_trait;
use glue::Endpoint as _;
use seed::{browser::fetch::FetchError, prelude::*};
#[async_trait(?Send)]
pub trait Endpoint: glue::Endpoint {
    async fn fetch() -> Result<Self, FetchError>;
}

#[async_trait(?Send)]
impl Endpoint for glue::Hello {
    async fn fetch() -> Result<Self, FetchError> {
        Request::new(Self::PATH).fetch().await?.json().await
    }
}
