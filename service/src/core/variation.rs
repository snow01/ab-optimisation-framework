use std::sync::atomic::AtomicU64;

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate)]
pub struct Variation {
    #[validate(length(min = 1))]
    pub name: String,

    #[validate(length(min = 1, max = 5))]
    pub short_name: String,

    #[validate(range(min = 1, max = 100))]
    pub size: u64,

    pub data: Option<JsonValue>,

    #[serde(default)]
    #[serde(skip)]
    pub picked_size: AtomicU64,
}

impl PartialEq for Variation {
    fn eq(&self, other: &Self) -> bool {
        (&self.name, &self.short_name, &self.size, &self.data)
            == (&other.name, &other.short_name, &other.size, &other.data)
    }
}
