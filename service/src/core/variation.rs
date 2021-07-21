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
