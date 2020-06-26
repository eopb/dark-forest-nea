use serde::{Deserialize, Serialize};
pub trait Endpoint {
    const PATH: &'static str;
}
