use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use serde::{Deserialize, Serialize};

use super::project::Project;
use super::skiplist_serde;

#[derive(Serialize, Deserialize)]
pub struct App {
    pub id: String,
    pub name: String,
    pub short_name: String,

    // todo: pub auth_key: String,
    #[serde(with = "skiplist_serde")]
    #[serde(default = "default_projects")]
    pub projects: SkipList<String, Project>,
}

fn default_projects() -> SkipList<String, Project> {
    SkipList::new(epoch::default_collector().clone())
}
