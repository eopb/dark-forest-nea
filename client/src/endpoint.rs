use std::fmt;

use {anyhow::anyhow, async_trait::async_trait, seed::prelude::*};

use shared::data::{
    Kind::{Binary, Json},
    KINDS,
};

/// Extension for REST endpoints that can be fetched using `GET`.
#[async_trait(?Send)]
pub trait Get: Endpoint + shared::Endpoint {
    async fn fetch() -> anyhow::Result<Self::Response> {
        let path = Self::path(KINDS);
        let fetch = Request::new(&path)
            .fetch()
            .await
            .map_err(Self::fetch_error)?;
        Self::get(fetch).await
    }
}

/// Extension for REST endpoints that can be fetched using `POST`.
#[async_trait(?Send)]
pub trait Post: Endpoint + shared::PostEndpoint {
    async fn fetch(post: Self::Requires) -> anyhow::Result<Self::Response> {
        let path = Self::path(KINDS);
        let fetch = Request::new(&path).method(Method::Post);
        let fetch = match KINDS.server_requires {
            Json => fetch.json(&post).map_err(Self::fetch_error)?,
            Binary => fetch.bytes(&bincode::serialize(&post).map_err(Self::fetch_error)?),
        }
        .fetch()
        .await
        .map_err(Self::fetch_error)?;
        Self::get(fetch).await
    }
}

/// Extension for all REST endpoints
#[async_trait(?Send)]
pub trait Endpoint: shared::Endpoint {
    /// Get data from a `Response`.
    async fn get(fetch: Response) -> anyhow::Result<Self::Response> {
        match KINDS.server_response {
            Json => fetch.json().await.map_err(Self::fetch_error),
            Binary => bincode::deserialize(&fetch.bytes().await.map_err(Self::fetch_error)?)
                .map_err(Self::fetch_error),
        }
    }
    /// Create error relating to some fetch process.
    ///
    /// To be used by `Result::map_err`.
    fn fetch_error(error: impl fmt::Debug + 'static) -> anyhow::Error {
        anyhow!(
            "Failed to fetch from Path: {} Error: {:?}",
            Self::path(KINDS),
            error
        )
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
