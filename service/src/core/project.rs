use serde::{Serialize, Deserialize};

use crate::core::experiment::Experiment;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub experiments: Vec<Experiment>
}