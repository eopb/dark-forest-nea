//! Data types sent over HTTP
pub mod create_account;
pub mod credentials;
pub mod hello;
pub mod new_project;
pub mod sign_out;
pub mod signed_in;
pub mod validation;

#[doc(inline)]
pub use validation::Validation;

/// A type that is related to a path.
pub trait Endpoint {
    /// Relative API path.
    const PATH: &'static str;
}
