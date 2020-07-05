use {
    anyhow::anyhow,
    async_trait::async_trait,
    seed::prelude::*,
    serde::{Deserialize, Serialize},
};

use shared::data::ResponseKind::{Binary, Json};

use crate::RESPONSE_KIND;

#[async_trait(?Send)]
pub trait Endpoint: 'static + shared::Endpoint + Serialize + for<'a> Deserialize<'a> {
    async fn fetch() -> anyhow::Result<Self> {
        let path = Self::path(RESPONSE_KIND);
        let fetch = Request::new(&path)
            .fetch()
            .await
            .map_err(|error| anyhow!("Failed to fetch: Path: {} Error: {:?}", &path, error))?;
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

impl Endpoint for shared::Hello {}
impl Endpoint for shared::SignedIn {}
impl Endpoint for shared::RefreshToken {}
