use std::collections::HashMap;

use {
    maplit::hashmap,
    serde::{Deserialize, Serialize},
};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Project {
    pub name: String,
    // TODO theme
    pub theme: (),
    // TODO use `indexmap` or alternative with user controlled ordering.
    pub chapters: HashMap<u32, Chapter>,
}

impl Project {
    // TODO should not be needed when users can create their own stories.
    pub fn example() -> Self {
        Self {
            name: "The Endless Loop!".to_owned(),
            theme: (),
            chapters: hashmap! {
                1 => Chapter {
                    heading: "This is the start of your adventure.".to_owned(),
                    body: "You can continue or leave.".to_owned(),
                    decisions: vec![
                        Decision {
                            body: "Continue".to_owned(),
                            goes_to: Some(Link::Chapter(2))
                        },
                        Decision {
                            body: "End".to_owned(),
                            goes_to: Some(Link::End)
                        }
                    ]
                },
                2 =>  Chapter {
                    heading: "This is the start of your adventure.".to_owned(),
                    body: "You may only continue".to_owned(),
                    decisions: vec![
                        Decision {
                            body: "Continue".to_owned(),
                            goes_to: Some(Link::Chapter(1))
                        }
                    ]
                }
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Chapter {
    pub heading: String,
    pub body: String,
    pub decisions: Vec<Decision>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Decision {
    pub body: String,
    pub goes_to: Option<Link>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Link {
    /// The story has ended.
    End,
    Chapter(u32),
}
