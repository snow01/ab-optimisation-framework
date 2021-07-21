use std::sync::atomic::AtomicU64;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
pub struct Variation {
    // #[serde(skip_deserializing)]
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub size: u64,
    pub data: Option<JsonValue>,

    #[serde(default)]
    #[serde(skip)]
    pub picked_size: AtomicU64,
}

impl PartialEq for Variation {
    fn eq(&self, other: &Self) -> bool {
        (
            &self.id,
            &self.name,
            &self.short_name,
            &self.size,
            &self.data,
        ) == (
            &other.id,
            &other.name,
            &other.short_name,
            &other.size,
            &other.data,
        )
    }
}
