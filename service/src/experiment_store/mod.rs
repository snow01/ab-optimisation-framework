use anyhow::Context;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use regex::Regex;

use crate::core::{App, AudienceList, Experiment, Project};
pub use crate::experiment_store::store::Store;
use crate::service::AbOptimisationService;
use crate::settings;

mod store;

lazy_static! {
    static ref APP_RE: Regex = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]app[.]data.json
",
    )
    .unwrap();
    static ref PROJECT_RE: Regex = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]
(?P<project_id>[A-Za-z0-9_~-]+) # project-id
[.]project[.]data[.]json
",
    )
    .unwrap();
    static ref EXPERIMENT_RE: Regex = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]
(?P<project_id>[A-Za-z0-9_~-]+) # project-id
[.]
(?P<experiment_id>[A-Za-z0-9_~-]+)   # experiment-id
[.]experiment[.]data[.]json
",
    )
    .unwrap();
    static ref AUDIENCE_LIST_RE: Regex = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
.
(?P<project_id>[A-Za-z0-9_~-]+) # project-id
.
(?P<list_id>[A-Za-z0-9_~-]+)   # list-id
.audience-list.data.json
",
    )
    .unwrap();
}

pub struct ExperimentStore {
    store: Store,
}

impl ExperimentStore {
    pub fn new_local_store(local_store_config: settings::LocalStoreConfig) -> anyhow::Result<ExperimentStore> {
        info!("Configuring local store: {:?}", local_store_config);
        let store = Store::new_local_store(local_store_config)?;
        Ok(ExperimentStore { store })
    }

    pub fn new_s3_store(s3_store_config: settings::S3StoreConfig) -> anyhow::Result<ExperimentStore> {
        info!("Configuring s3 store: {:?}", s3_store_config);
        let store = Store::new_s3_store(s3_store_config)?;
        Ok(ExperimentStore { store })
    }

    pub(crate) fn load_apps(&self, service: &AbOptimisationService) -> anyhow::Result<()> {
        self.store.visit_path(&self.apps_path(), |data_path, last_modified_time| {
            let f_name = data_path.to_string_lossy();
            // info!("Got file: {}", f_name);

            if let Some(matches) = APP_RE.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();

                // info!("Got app_id: {}", app_id);

                let app: App = self.store.read_data(data_path)?;
                service.load_app(app_id, app, last_modified_time)?;
            }

            Ok(())
        })
    }

    pub(crate) fn load_projects(&self, service: &AbOptimisationService) -> anyhow::Result<()> {
        self.store.visit_path(&self.projects_path(), |data_path, last_modified_time| {
            let f_name = data_path.to_string_lossy();
            // info!("Got file: {}", f_name);

            if let Some(matches) = PROJECT_RE.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();
                let project_id = matches.name("project_id").unwrap().as_str();

                // info!("Got app_id: {} and project_id: {}", app_id, project_id);

                let project: Project = self.store.read_data(data_path)?;
                service
                    .load_project(&f_name, app_id, project_id, project, last_modified_time)
                    .with_context(|| format!("Error in adding project for file: {}", data_path.to_string_lossy()))?;
            }

            Ok(())
        })
    }

    pub(crate) fn load_experiments(&self, service: &AbOptimisationService) -> anyhow::Result<()> {
        self.store.visit_path(&self.experiments_path(), |data_path, last_modified_time| {
            let f_name = data_path.to_string_lossy();

            if let Some(matches) = EXPERIMENT_RE.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();
                let project_id = matches.name("project_id").unwrap().as_str();
                let experiment_id = matches.name("experiment_id").unwrap().as_str();

                // info!("Got app_id: {} and project_id: {}, experiment_id: {}", app_id, project_id, experiment_id);

                let experiment: Experiment = self.store.read_data(data_path)?;
                service
                    .load_experiment(&f_name, app_id, project_id, experiment_id, experiment, last_modified_time)
                    .with_context(|| format!("Error in adding experiment for file: {}", data_path.to_string_lossy()))?;
            }

            Ok(())
        })
    }

    pub(crate) fn load_audience_lists(&self, service: &AbOptimisationService) -> anyhow::Result<()> {
        self.store.visit_path(&self.audience_lists_path(), |data_path, last_modified_time| {
            let f_name = data_path.to_string_lossy();

            if let Some(matches) = AUDIENCE_LIST_RE.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();
                let project_id = matches.name("project_id").unwrap().as_str();
                let list_id = matches.name("list_id").unwrap().as_str();

                // info!("Got app_id: {} and project_id: {}, list_id: {}", app_id, project_id, list_id);

                let audience_list: AudienceList = self.store.read_data(data_path)?;
                service
                    .load_audience_list(&f_name, app_id, project_id, list_id, audience_list, last_modified_time)
                    .with_context(|| format!("Error in adding audience list for file: {}", data_path.to_string_lossy()))?;
            }

            Ok(())
        })
    }

    pub(crate) fn write_app_data(&self, app: &App) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.app.data.json", self.apps_path(), &app.id);

        info!("Writing app data to file: {}", file_path);

        self.store.write_data(app, &file_path)
    }

    pub(crate) fn write_project_data(&self, app_id: &str, proj: &Project) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.{}.project.data.json", self.projects_path(), app_id, &proj.id);

        info!("Writing project data to file: {}", file_path);

        self.store.write_data(proj, &file_path)
    }

    pub(crate) fn write_experiment_data(&self, app_id: &str, project_id: &str, experiment: &Experiment) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.{}.{}.experiment.data.json", self.experiments_path(), app_id, project_id, &experiment.id);

        info!("Writing experiment data to file: {}", file_path);

        self.store.write_data(experiment, &file_path)
    }

    pub(crate) fn write_audience_list_data(&self, app_id: &str, project_id: &str, audience_list: &AudienceList) -> anyhow::Result<()> {
        let file_path = format!(
            "{}/{}.{}.{}.audience-list.data.json",
            self.audience_lists_path(),
            app_id,
            project_id,
            &audience_list.id
        );

        info!("Writing audience_list data to file: {}", file_path);

        self.store.write_data(audience_list, &file_path)
    }

    fn apps_path(&self) -> String {
        format!("{}/apps", self.store.path())
    }

    fn projects_path(&self) -> String {
        format!("{}/projects", self.store.path())
    }

    fn experiments_path(&self) -> String {
        format!("{}/experiments", self.store.path())
    }

    fn audience_lists_path(&self) -> String {
        format!("{}/audience_lists", self.store.path())
    }
}
