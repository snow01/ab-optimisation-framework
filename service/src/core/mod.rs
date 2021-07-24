use serde::{Deserialize, Serialize};

pub use app::App;
pub use audience_list::AudienceList;
pub use experiment::Audience;
pub use experiment::AudienceSpec;
pub use experiment::Experiment;
pub use experiment::SizeSpec;
pub use project::Project;
pub use project::TrackingMethod;
pub use variation::Variation;

pub mod app;
mod audience_list;
mod experiment;
mod experiment_group;
mod project;
mod skiplist_serde;
mod variation;

pub trait HasId {
    fn id(&self) -> &str;
}

#[derive(Serialize, Deserialize)]
pub struct AddResponse {
    pub id: String,
}
