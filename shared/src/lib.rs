//! `shared` is depended on by both the client and server.
//!
//! `shared` defines all types serialized, deserialized and sent over HTTP along with other types required by both the client and server

#![deny(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::missing_errors_doc,
    clippy::enum_glob_use,
    clippy::must_use_candidate
)]

pub mod data;
pub mod qs;
pub mod routes;

#[doc(inline)]
pub use {
    data::{
        create_account::CreateAccount, credentials::Credentials, hello::Hello, sign_out::SignOut,
        signed_in::SignedIn, Endpoint,
    },
    routes::Route,
};
