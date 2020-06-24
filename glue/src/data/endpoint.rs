use serde::{Deserialize, Serialize};
pub trait Endpoint: Serialize + for<'a> Deserialize<'a> {
    const PATH: &'static str;
}
