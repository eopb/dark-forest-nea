use std::{convert::TryInto, ops::Index, ops::IndexMut};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Chapters(Vec<Chapter>);

impl Chapters {
    pub fn iter(&self) -> impl Iterator<Item = &Chapter> {
        self.0.iter()
    }
    pub fn get_chapter_with_key(&self, key: u32) -> Option<&Chapter> {
        self.iter().find(|chapter| chapter.key == key)
    }
    pub fn example() -> Self {
        Self(vec![
            Chapter {
                key: 1,
                heading: "This is the start of your adventure.".to_owned(),
                body: "You can continue or leave.".to_owned(),
                decisions: vec![
                    Decision {
                        body: "Continue".to_owned(),
                        goes_to: Some(Link::Chapter(2)),
                    },
                    Decision {
                        body: "End".to_owned(),
                        goes_to: Some(Link::End),
                    },
                ],
            },
            Chapter {
                key: 2,
                heading: "This is the start of your adventure.".to_owned(),
                body: "You may only continue".to_owned(),
                decisions: vec![Decision {
                    body: "Continue".to_owned(),
                    goes_to: Some(Link::Chapter(1)),
                }],
            },
        ])
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn get_chapter_with_key() {
        Chapters::example().get_chapter_with_key(2).unwrap();
    }
}

impl Index<usize> for Chapters {
    type Output = Chapter;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Chapters {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Chapter {
    // MongoDB does not have support for unsized types so we have to custom serialize them.
    #[serde(serialize_with = "serialize_key", deserialize_with = "deserialize_key")]
    pub key: u32,
    pub heading: String,
    pub body: String,
    pub decisions: Vec<Decision>,
}

#[allow(clippy::trivially_copy_pass_by_ref)]
fn serialize_key<S>(key: &u32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    i64::from(*key).serialize(serializer)
}

fn deserialize_key<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    i64::deserialize(deserializer).map(|key| {
        key.try_into()
            .expect("Malformed data on mongo. Int overflow.")
    })
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
