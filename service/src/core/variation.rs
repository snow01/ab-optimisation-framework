use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::atomic::AtomicI64;

#[derive(Serialize, Deserialize)]
pub struct Variation {
    pub id: String,
    pub name: String,
    pub short_name: String,
    pub size: i64,
    pub data: Option<JsonValue>,

    #[serde(default)]
    #[serde(skip)]
    pub picked_size: AtomicI64,
}
