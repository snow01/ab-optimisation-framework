use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use anyhow::{Context, Error};
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use parking_lot::RwLock;
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::config::settings;
use crate::core::{App, AudienceList, Experiment, Project};
use crate::server::ServiceBuilder;

// #[derive(Clone)]
pub struct AbOptimisationService {
    pub apps: SkipList<String, RwLock<App>>,
}

pub struct AbOptimisationServiceBuilder {}

impl ServiceBuilder<AbOptimisationService> for AbOptimisationServiceBuilder {
    fn build(self) -> anyhow::Result<AbOptimisationService> {
        let service = AbOptimisationService {
            apps: SkipList::new(epoch::default_collector().clone()),
        };

        load_all_apps(&service)?;
        load_all_projects(&service)?;
        load_all_experiments(&service)?;
        load_all_audience_lists(&service)?;

        Ok(service)
    }
}

impl AbOptimisationService {
    pub fn load_app(&self, app_id: &str, mut app: App) -> anyhow::Result<()> {
        info!("Loading app for id:{}", app_id);

        let guard = &epoch::pin();
        app.id = app_id.to_string();

        self.apps
            .insert(app_id.to_string(), RwLock::new(app), guard);

        Ok(())
    }

    pub fn write_app_data(&self, app: &App) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.app.data.json", apps_directory(), &app.id);

        info!("Writing app data to file: {}", file_path);

        write_helper(app, file_path)
    }

    pub fn load_project(
        &self,
        file: &str,
        app_id: &str,
        project_id: &str,
        mut project: Project,
    ) -> anyhow::Result<()> {
        info!("Loading project for app:{}, id:{}", app_id, project_id);

        let guard = &epoch::pin();
        project.id = project_id.to_string();

        self.visit_app(app_id, guard, |entry| {
            entry.value().read().projects.insert(
                project_id.to_string(),
                RwLock::new(project),
                guard,
            );

            Ok(())
        })
        .with_context(|| format!("Error in loading project from file: {}", file))
    }

    pub fn write_project_data(&self, app_id: &str, proj: &Project) -> anyhow::Result<()> {
        let file_path = format!(
            "{}/{}.{}.project.data.json",
            projects_directory(),
            app_id,
            &proj.id
        );

        info!("Writing project data to file: {}", file_path);

        write_helper(proj, file_path)
    }

    pub fn load_experiment(
        &self,
        file: &str,
        app_id: &str,
        project_id: &str,
        experiment_id: &str,
        mut experiment: Experiment,
    ) -> anyhow::Result<()> {
        info!(
            "Loading experiment fo app:{}, project:{}, id:{}",
            app_id, project_id, experiment_id
        );

        let guard = &epoch::pin();
        experiment.id = experiment_id.to_string();

        self.visit_project(app_id, project_id, guard, |entry| {
            entry.value().read().experiments.insert(
                experiment_id.to_string(),
                RwLock::new(experiment),
                guard,
            );

            Ok(())
        })
        .with_context(|| format!("Error in loading experiment from file: {}", file))
    }

    pub fn write_experiment_data(
        &self,
        app_id: &str,
        project_id: &str,
        experiment: &Experiment,
    ) -> anyhow::Result<()> {
        let file_path = format!(
            "{}/{}.{}.{}.experiment.data.json",
            experiments_directory(),
            app_id,
            project_id,
            &experiment.id
        );

        info!("Writing experiment data to file: {}", file_path);

        write_helper(experiment, file_path)
    }

    pub fn load_audience_list(
        &self,
        file: &str,
        app_id: &str,
        project_id: &str,
        list_id: &str,
        mut audience_list: AudienceList,
    ) -> anyhow::Result<()> {
        info!(
            "Loading audience_list for app:{}, project:{}, list_id:{}",
            app_id, project_id, list_id
        );

        let guard = &epoch::pin();
        audience_list.id = list_id.to_string();

        self.visit_project(app_id, project_id, guard, |entry| {
            entry.value().read().audience_lists.insert(
                list_id.to_string(),
                RwLock::new(audience_list),
                guard,
            );

            Ok(())
        })
        .with_context(|| format!("Error in loading audience list from file: {}", file))
    }

    pub fn write_audience_list_data(
        &self,
        app_id: &str,
        project_id: &str,
        audience_list: &AudienceList,
    ) -> anyhow::Result<()> {
        let file_path = format!(
            "{}/{}.{}.{}.audience-list.data.json",
            audience_lists_directory(),
            app_id,
            project_id,
            &audience_list.id
        );

        info!("Writing audience_list data to file: {}", file_path);

        write_helper(audience_list, file_path)
    }
}

fn apps_directory() -> String {
    settings()
        .read()
        .get::<String>("apps_directory")
        .unwrap_or_else(|_| format!("data/apps"))
}

fn projects_directory() -> String {
    settings()
        .read()
        .get::<String>("projects_directory")
        .unwrap_or_else(|_| format!("data/projects"))
}

fn experiments_directory() -> String {
    settings()
        .read()
        .get::<String>("experiments_directory")
        .unwrap_or_else(|_| format!("data/experiments"))
}

fn audience_lists_directory() -> String {
    settings()
        .read()
        .get::<String>("audience_lists_directory")
        .unwrap_or_else(|_| format!("data/audience_lists"))
}

fn load_all_apps(service: &AbOptimisationService) -> anyhow::Result<()> {
    let re = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]app[.]data.json
",
    )
    .unwrap();

    for entry in WalkDir::new(apps_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        // info!("Got file: {}", f_name);

        if let Some(matches) = re.captures(&f_name) {
            let app_id = matches.name("app_id").unwrap().as_str();

            // info!("Got app_id: {}", app_id);

            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let app: App = serde_json::from_reader(reader).with_context(|| {
                format!(
                    "Error in parsing app from file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
            service.load_app(app_id, app)?;
        }
    }

    Ok(())
}

fn load_all_projects(service: &AbOptimisationService) -> anyhow::Result<()> {
    let re = Regex::new(
        r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]
(?P<project_id>[A-Za-z0-9_~-]+) # project-id
[.]project[.]data[.]json
",
    )
    .unwrap();

    for entry in WalkDir::new(projects_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        // info!("Got file: {}", f_name);

        if let Some(matches) = re.captures(&f_name) {
            let app_id = matches.name("app_id").unwrap().as_str();
            let project_id = matches.name("project_id").unwrap().as_str();

            // info!("Got app_id: {} and project_id: {}", app_id, project_id);

            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let project: Project = serde_json::from_reader(reader).with_context(|| {
                format!(
                    "Error in parsing project from file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
            service
                .load_project(&f_name, app_id, project_id, project)
                .with_context(|| {
                    format!(
                        "Error in adding project for file: {}",
                        entry.path().to_string_lossy()
                    )
                })?;
        }
    }

    Ok(())
}

fn load_all_experiments(service: &AbOptimisationService) -> anyhow::Result<()> {
    let re = Regex::new(
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

    for entry in WalkDir::new(experiments_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        // info!("Got file: {}", f_name);

        if let Some(matches) = re.captures(&f_name) {
            let app_id = matches.name("app_id").unwrap().as_str();
            let project_id = matches.name("project_id").unwrap().as_str();
            let experiment_id = matches.name("experiment_id").unwrap().as_str();

            // info!(
            //     "Got app_id: {} and project_id: {}, experiment_id: {}",
            //     app_id, project_id, experiment_id
            // );

            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let experiment: Experiment = serde_json::from_reader(reader).with_context(|| {
                format!(
                    "Error in parsing experiment from file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
            service
                .load_experiment(&f_name, app_id, project_id, experiment_id, experiment)
                .with_context(|| {
                    format!(
                        "Error in adding experiment for file: {}",
                        entry.path().to_string_lossy()
                    )
                })?;
        }
        // if f_name.matches("{}-{}-{}-experiment-data.json") {
        //
        // }
    }

    Ok(())
}

fn load_all_audience_lists(service: &AbOptimisationService) -> anyhow::Result<()> {
    let re = Regex::new(
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

    for entry in WalkDir::new(audience_lists_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        // info!("Got file: {}", f_name);

        if let Some(matches) = re.captures(&f_name) {
            let app_id = matches.name("app_id").unwrap().as_str();
            let project_id = matches.name("project_id").unwrap().as_str();
            let list_id = matches.name("list_id").unwrap().as_str();

            // info!(
            //     "Got app_id: {} and project_id: {}, list_id: {}",
            //     app_id, project_id, list_id
            // );

            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let audience_list: AudienceList =
                serde_json::from_reader(reader).with_context(|| {
                    format!(
                        "Error in parsing audience list from file: {}",
                        entry.path().to_string_lossy()
                    )
                })?;
            service
                .load_audience_list(&f_name, app_id, project_id, list_id, audience_list)
                .with_context(|| {
                    format!(
                        "Error in adding audience list for file: {}",
                        entry.path().to_string_lossy()
                    )
                })?;
        }
    }

    Ok(())
}

fn write_helper<T>(data: &T, file_path: String) -> Result<(), Error>
where
    T: Serialize,
{
    let file = File::create(&file_path)?;
    let mut writer = BufWriter::new(file);

    // Write the JSON contents to the file.
    serde_json::to_writer(&mut writer, data)
        .with_context(|| format!("Error in writing data to file: {}", file_path))?;

    writer.flush()?;

    Ok(())
}
