//! Shared application state.

#[derive(Debug)]
pub struct State;

impl State {
    /// Create a new instance of `State`.
    pub async fn new() -> tide::Result<Self> {
        Ok(Self)
    }
}
