use std::ops::Deref;

use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use parking_lot::RwLock;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::core::HasId;
use serde::ser::SerializeSeq;

// pub fn skiplist_to_vec<'g, T>(value: &SkipList<String, RwLock<T>>, guard: &'g Guard) -> Vec<&'g T> {
//     let mut list = Vec::<&T>::new();
//
//     for entry in value.iter(guard) {
//         list.push(entry.value());
//     }
//
//     list
// }

#[derive(Serialize)]
pub struct SerdeListWrapper<'a, T>(#[serde(serialize_with = "serialize")] pub &'a SkipList<String, RwLock<T>>)
where
    T: Serialize + Send + 'static;

pub fn serialize<T, S>(value: &SkipList<String, RwLock<T>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Send + 'static,
{
    let guard = &epoch::pin();

    let mut seq = serializer.serialize_seq(Some(value.len()))?;

    for entry in value.iter(guard) {
        let value_lock = entry.value();
        let value_guard = value_lock.read();
        seq.serialize_element(value_guard.deref())?;
    }

    seq.end()
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<SkipList<String, RwLock<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Send + 'static + HasId,
{
    let data = SkipList::<String, RwLock<T>>::new(epoch::default_collector().clone());
    let list = Vec::<T>::deserialize(deserializer)?;

    let guard = &epoch::pin();
    for value in list {
        data.insert(value.id().to_string(), RwLock::new(value), guard);
    }

    Ok(data)
}
