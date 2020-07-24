//! Useful functions not complex enough for their own modules.

use std::{
    collections::HashMap,
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Wrapper type for `HashMaps` that serializes keys as strings to better
/// support `MongoDB`.
#[derive(Clone, Debug, Default)]
pub struct StringMap<K, V>(HashMap<K, V>);

#[allow(clippy::type_repetition_in_bounds)]
impl<K, V> Serialize for StringMap<K, V>
where
    K: ToString,
    HashMap<K, V>: Clone,
    HashMap<String, V>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0
            .clone()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<HashMap<String, V>>()
            .serialize(serializer)
    }
}

impl<'de, K, V> Deserialize<'de> for StringMap<K, V>
where
    K: FromStr + Eq + Hash,
    <K as FromStr>::Err: Debug,
    HashMap<String, V>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = HashMap::<String, V>::deserialize(deserializer)?;
        Ok(Self(
            inner
                .into_iter()
                .map(|(k, v)| (k.parse().unwrap(), v))
                .collect(),
        ))
    }
}

impl<K, V> Deref for StringMap<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for StringMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> From<HashMap<K, V>> for StringMap<K, V> {
    fn from(m: HashMap<K, V>) -> Self {
        Self(m)
    }
}
