//! Data types sent over HTTP
pub mod create_account;
pub mod credentials;
pub mod hello;
pub mod new_project;
pub mod refresh_token;
pub mod sign_out;
pub mod signed_in;
pub mod validation;

#[doc(inline)]
pub use validation::Validation;

/// Enum to specify in what format data should be sent between client and server.
#[derive(Copy, Clone)]
pub enum ResponseKind {
    Json,
    Binary,
}
use ResponseKind::{Binary, Json};

/// A type that is related to a path.
pub trait Endpoint {
    /// Relative API path.
    ///
    /// This path will be nested in a response kind.
    const PATH: &'static str;

    /// Full relative path for this endpoint with a given response body type.
    fn path(res_kind: ResponseKind) -> String {
        format!(
            "/{}{}",
            match res_kind {
                Binary => "api/bin",
                Json => "api/json",
            },
            Self::PATH
        )
    }
}
