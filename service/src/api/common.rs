use std::fmt::{Display, Formatter};

use json_value_merge::Merge;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone, Serialize, Deserialize)]
pub enum ExperimentMemberKind {
    #[serde(rename = "C")]
    Control,

    #[serde(rename = "T")]
    Test,
}

impl Display for ExperimentMemberKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ExperimentMemberKind::Control => {
                write!(f, "C")
            }
            ExperimentMemberKind::Test => {
                write!(f, "T")
            }
        }
    }
}

pub fn merge_data(experiment_data: Option<serde_json::Value>, variation_data: Option<serde_json::Value>) -> Option<serde_json::Value> {
    match (experiment_data, variation_data) {
        (None, None) => None,
        (Some(data), None) | (None, Some(data)) => Some(data.clone()),
        (Some(mut experiment_data), Some(variation_data)) => {
            experiment_data.merge(variation_data);

            Some(experiment_data)
        }
    }
}
