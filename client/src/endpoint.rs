use std::fmt;

use {anyhow::anyhow, async_trait::async_trait, seed::prelude::*};

use shared::endpoint::{
    create_account::CreateAccount,
    hello::Hello,
    new_project::NewProject,
    refresh_token::RefreshToken,
    sign_in::SignIn,
    signed_in::SignedIn,
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

impl Get for Hello {}
impl Endpoint for Hello {}

impl Post for SignedIn {}
impl Endpoint for SignedIn {}

impl Post for RefreshToken {}
impl Endpoint for RefreshToken {}

impl Post for SignIn {}
impl Endpoint for SignIn {}

impl Post for CreateAccount {}
impl Endpoint for CreateAccount {}

impl Post for NewProject {}
impl Endpoint for NewProject {}
