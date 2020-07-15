use {anyhow::anyhow, async_trait::async_trait, seed::prelude::*};

use shared::data::{
    DataKind::{Binary, Json},
    DATA_KINDS,
};

/// Extension for endpoints that can be fetched from the server
#[async_trait(?Send)]
pub trait Get: Endpoint + shared::Endpoint {
    async fn fetch() -> anyhow::Result<Self::Response> {
        let path = Self::path(DATA_KINDS);
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
        let path = Self::path(DATA_KINDS);
        let fetch = Request::new(&path).method(Method::Post);
        let data = bincode::serialize(&post)
            .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?;
        let fetch = match DATA_KINDS.server_requires {
            Json => fetch
                .header(Header::custom("Accept-Language", "en"))
                .json(&post)
                .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?,
            Binary => bytes(fetch, &data),
        }
        .fetch()
        .await
        .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?;
        Self::get(fetch).await
    }
}
pub fn bytes<'a>(req: Request<'a>, data: &'a [u8]) -> Request<'a> {
    req.body(js_sys::Uint8Array::from(data).into())
        .header(Header::content_type("application/octet-stream"))
}

#[async_trait(?Send)]
pub trait Endpoint: shared::Endpoint {
    async fn get(fetch: Response) -> anyhow::Result<Self::Response> {
        let path = Self::path(DATA_KINDS);
        match DATA_KINDS.server_response {
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
