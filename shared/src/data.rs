pub mod chapters;

pub use chapters::Chapters;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    pub description: String,
    // TODO theme
    pub theme: (),
    // TODO use `indexmap` or alternative with user controlled ordering.
    pub chapters: Chapters,
}

impl Project {
    // TODO should not be needed when users can create their own stories.
    pub fn example() -> Self {
        Self {
            name: "The Endless Loop!".to_owned(),
            theme: (),
            description: "A stupid game".to_owned(),
            chapters: Chapters::example(),
        }
    }
}
