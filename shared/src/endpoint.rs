//! Data types sent over HTTP

pub mod endpoints;

pub use endpoints::{create_account, edit, hello, new_project, refresh_token, sign_in, signed_in};

use serde::{Deserialize, Serialize};

/// The kind of body to request from and send to server endpoints.
pub const KINDS: Kinds = Kinds {
    server_requires: Binary,
    server_response: Binary,
};

/// Struct to specify in what format data should be sent between client and
/// server.
#[derive(Copy, Clone)]
pub struct Kinds {
    pub server_response: Kind,
    pub server_requires: Kind,
}

impl Kinds {
    /// All possible server setups.
    pub const fn possible() -> &'static [Self; 4] {
        &[
            Self {
                server_requires: Binary,
                server_response: Binary,
            },
            Self {
                server_requires: Json,
                server_response: Binary,
            },
            Self {
                server_requires: Binary,
                server_response: Json,
            },
            Self {
                server_requires: Json,
                server_response: Json,
            },
        ]
    }
}

/// The format to send data in.
#[derive(Copy, Clone)]
pub enum Kind {
    Json,
    Binary,
}
use Kind::{Binary, Json};

impl Kind {
    const fn path(&self) -> &str {
        match self {
            Binary => "/bin",
            Json => "/json",
        }
    }
}

/// A REST Endpoint
pub trait Endpoint: 'static {
    /// The data that this endpoint responds with.
    type Response: for<'a> Deserialize<'a> + Serialize;
    /// Relative API path.
    ///
    /// This path will be nested in a response kind.
    const PATH: &'static str;

    /// Full relative path for the version of this endpoint accepting given data
    /// format.
    fn path(data_kinds: Kinds) -> String {
        format!(
            "/api{}{}{}",
            data_kinds.server_requires.path(),
            data_kinds.server_response.path(),
            Self::PATH
        )
    }
}

/// A REST Endpoint that can be `POST`ed to.
///
/// If an Endpoint does not implement this it is `Get`.
#[allow(clippy::module_name_repetitions)]
pub trait PostEndpoint: Endpoint {
    /// The data that this endpoint requires to process.
    type Requires: for<'a> Deserialize<'a> + Serialize + Send + Sync + 'static;
}
