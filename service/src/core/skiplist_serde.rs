use crate::core::HasId;
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<T, S>(value: &SkipList<String, T>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Serialize + Send + 'static,
{
    let guard = &epoch::pin();

    let mut list = Vec::<&T>::new();

    for entry in value.iter(guard) {
        list.push(entry.value());
    }

    list.serialize(serializer)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<SkipList<String, T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Send + 'static + HasId,
{
    let data = SkipList::<String, T>::new(epoch::default_collector().clone());
    let list = Vec::<T>::deserialize(deserializer)?;

    let guard = &epoch::pin();
    for value in list {
        data.insert(value.id().to_string(), value, guard);
    }

    Ok(data)
}
