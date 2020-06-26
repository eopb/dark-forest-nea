pub mod create_account;
pub mod credentials;
pub mod hello;
pub mod sign_out;
pub mod signed_in;

pub trait Endpoint {
    const PATH: &'static str;
}
