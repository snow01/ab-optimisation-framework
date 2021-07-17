pub use app::App;
pub use experiment::Audience;
pub use experiment::AudienceSpec;
pub use experiment::Experiment;
pub use experiment::SizeSpec;
pub use project::AudienceList;
pub use project::Project;
pub use variation::Variation;

mod app;
mod experiment;
mod project;
mod skiplist_serde;
mod variation;

pub trait HasId {
    fn id(&self) -> &str;
}
