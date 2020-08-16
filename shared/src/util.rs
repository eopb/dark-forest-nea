//! Useful functions not complex enough for their own modules.

use std::{
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
    str::FromStr,
};

use {
    num_traits::int::PrimInt,
    serde::{Deserialize, Deserializer, Serialize, Serializer},
    indexmap::IndexMap
};

/// Wrapper type for `HashMaps` that serializes keys as strings to better
/// support `MongoDB`.
#[derive(Clone, Debug, Default)]
pub struct StringMap<K, V>(IndexMap<K, V>);

#[allow(clippy::type_repetition_in_bounds)]
impl<K, V> Serialize for StringMap<K, V>
where
    K: ToString + PrimInt,
    IndexMap<K, V>: Clone,
    IndexMap<String, V>: Serialize ,
    // <IndexMap<String, V> as Serialize>::Ok: Debug ,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
       self.0
            .clone()
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<IndexMap<String, V>>()
            .serialize(serializer )
    }
}

impl<'de, K, V> Deserialize<'de> for StringMap<K, V>
where
    K: FromStr + Eq + Hash + PrimInt,
    <K as FromStr>::Err: Debug,
    IndexMap<String, V>: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let inner = IndexMap::<String, V>::deserialize(deserializer)?;
        Ok(Self(
            inner
                .into_iter()
                .map(|(k, v)| (k.parse().unwrap(), v))
                .collect(),
        ))
    }
}

impl<K, V> Deref for StringMap<K, V> {
    type Target = IndexMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<K, V> DerefMut for StringMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<K, V> From<IndexMap<K, V>> for StringMap<K, V> {
    fn from(m: IndexMap<K, V>) -> Self {
        Self(m)
    }
}
