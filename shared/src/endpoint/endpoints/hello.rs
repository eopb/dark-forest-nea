use crate::Endpoint;

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hello;

/// Simple blob of data used for testing purposes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Res {
    pub msg: String,
}

impl Endpoint for Hello {
    type Response = Res;
    const PATH: &'static str = "/hello";
}
