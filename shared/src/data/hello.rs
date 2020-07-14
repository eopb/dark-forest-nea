use crate::Endpoint;

use serde::{Deserialize, Serialize};

/// Simple blob of data used for testing purposes.
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Hello {
    pub msg: String,
}

impl Endpoint for Hello {
    type Response = ();
    const PATH: &'static str = "/hello";
}
