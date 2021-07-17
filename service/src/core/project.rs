use std::collections::HashSet;

use crossbeam_skiplist::SkipList;
use serde::{Deserialize, Serialize};

use crate::core::HasId;

use super::experiment::Experiment;
use super::skiplist_serde;

#[derive(Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub app: String,
    pub name: String,
    pub short_name: String,

    #[serde(with = "skiplist_serde")]
    pub experiments: SkipList<String, Experiment>,

    #[serde(with = "skiplist_serde")]
    pub audience_lists: SkipList<String, AudienceList>,
}

impl HasId for Project {
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct AudienceList {
    pub id: String,
    pub name: String,
    pub list: HashSet<String>,
}

impl HasId for AudienceList {
    fn id(&self) -> &str {
        &self.id
    }
}

// pub struct ExperimentGroup {
//     pub id: String,
//     pub
// }
