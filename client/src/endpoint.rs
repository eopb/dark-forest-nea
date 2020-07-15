use {anyhow::anyhow, async_trait::async_trait, seed::prelude::*};

use shared::data::ResponseKind::{Binary, Json};

use crate::RESPONSE_KIND;

/// Extension for endpoints that can be fetched from the server
#[async_trait(?Send)]
pub trait Get: Endpoint + shared::Endpoint {
    async fn fetch() -> anyhow::Result<Self::Response> {
        let path = Self::path(RESPONSE_KIND);
        let fetch = Request::new(&path)
            .fetch()
            .await
            .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?;
        Self::get(fetch).await
    }
}

#[async_trait(?Send)]
pub trait Post: Endpoint + shared::PostEndpoint {
    async fn fetch(post: Self::Requires) -> anyhow::Result<Self::Response> {
        let path = Self::path(RESPONSE_KIND);
        let fetch = Request::new(&path)
            .method(Method::Post)
            .header(Header::custom("Accept-Language", "en"))
            .json(&post)
            .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?
            .fetch()
            .await
            .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?;
        Self::get(fetch).await
    }
}

#[async_trait(?Send)]
pub trait Endpoint: shared::Endpoint {
    async fn get(fetch: Response) -> anyhow::Result<Self::Response> {
        let path = Self::path(RESPONSE_KIND);
        match RESPONSE_KIND {
            Json => fetch
                .json()
                .await
                .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", path, error)),
            Binary => bincode::deserialize(&fetch.bytes().await.map_err(|error| {
                anyhow!(
                    "Failed to fetch binary from Path: {} Error: {:?}",
                    path,
                    error
                )
            })?)
            .map_err(|error| {
                anyhow!(
                    "Failed to parse binary from Path: {} Error: {:?}",
                    path,
                    error
                )
            }),
        }
    }
}
impl Get for shared::Hello {}
impl Endpoint for shared::Hello {}

impl Post for shared::SignedIn {}
impl Endpoint for shared::SignedIn {}

impl Post for shared::RefreshToken {}
impl Endpoint for shared::RefreshToken {}

impl Post for shared::SignIn {}
impl Endpoint for shared::SignIn {}

impl Post for shared::CreateAccount {}
impl Endpoint for shared::CreateAccount {}

impl Post for shared::NewProject {}
impl Endpoint for shared::NewProject {}
