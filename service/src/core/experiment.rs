use serde::{Serialize, Deserialize};

use crate::core::variation::Variation;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experiment {
    pub variations: Vec<Variation>,
}