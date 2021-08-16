use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use anyhow::Context;
use regex::Regex;
use serde::Serialize;
use walkdir::WalkDir;

use crate::config::settings;
use crate::core::{App, AudienceList, Experiment, Project};
use crate::service::AbOptimisationService;

pub trait ExperimentStore {
    fn write_data<T>(data: &T, file_path: String) -> anyhow::Result<()>
    where
        T: Serialize;

    fn load_all_apps(service: &AbOptimisationService) -> anyhow::Result<()> {
        let re = Regex::new(
            r"(?x)
(?P<app_id>[A-Za-z0-9_~-]+)  # app-id
[.]app[.]data.json
",
        )
        .unwrap();

        for entry in WalkDir::new(apps_directory()).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            // info!("Got file: {}", f_name);

            if let Some(matches) = re.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();

                // info!("Got app_id: {}", app_id);

                let file = File::open(entry.path())?;
                let reader = BufReader::new(file);

                // Read the JSON contents of the file as an instance of `User`.
                let app: App =
                    serde_json::from_reader(reader).with_context(|| format!("Error in parsing app from file: {}", entry.path().to_string_lossy()))?;
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

        for entry in WalkDir::new(projects_directory()).follow_links(true).into_iter().filter_map(|e| e.ok()) {
            let f_name = entry.file_name().to_string_lossy();
            // info!("Got file: {}", f_name);

            if let Some(matches) = re.captures(&f_name) {
                let app_id = matches.name("app_id").unwrap().as_str();
                let project_id = matches.name("project_id").unwrap().as_str();

                // info!("Got app_id: {} and project_id: {}", app_id, project_id);

                let file = File::open(entry.path())?;
                let reader = BufReader::new(file);

                // Read the JSON contents of the file as an instance of `User`.
                let project: Project =
                    serde_json::from_reader(reader).with_context(|| format!("Error in parsing project from file: {}", entry.path().to_string_lossy()))?;
                service
                    .load_project(&f_name, app_id, project_id, project)
                    .with_context(|| format!("Error in adding project for file: {}", entry.path().to_string_lossy()))?;
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

        for entry in WalkDir::new(experiments_directory()).follow_links(true).into_iter().filter_map(|e| e.ok()) {
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
                let experiment: Experiment =
                    serde_json::from_reader(reader).with_context(|| format!("Error in parsing experiment from file: {}", entry.path().to_string_lossy()))?;
                service
                    .load_experiment(&f_name, app_id, project_id, experiment_id, experiment)
                    .with_context(|| format!("Error in adding experiment for file: {}", entry.path().to_string_lossy()))?;
            }
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

        for entry in WalkDir::new(audience_lists_directory()).follow_links(true).into_iter().filter_map(|e| e.ok()) {
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
                    serde_json::from_reader(reader).with_context(|| format!("Error in parsing audience list from file: {}", entry.path().to_string_lossy()))?;
                service
                    .load_audience_list(&f_name, app_id, project_id, list_id, audience_list)
                    .with_context(|| format!("Error in adding audience list for file: {}", entry.path().to_string_lossy()))?;
            }
        }

        Ok(())
    }

    fn write_app_data(&self, app: &App) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.app.data.json", apps_directory(), &app.id);

        info!("Writing app data to file: {}", file_path);

        write_helper(app, file_path)
    }

    fn write_project_data(&self, app_id: &str, proj: &Project) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.{}.project.data.json", projects_directory(), app_id, &proj.id);

        info!("Writing project data to file: {}", file_path);

        write_helper(proj, file_path)
    }

    fn write_experiment_data(&self, app_id: &str, project_id: &str, experiment: &Experiment) -> anyhow::Result<()> {
        let file_path = format!("{}/{}.{}.{}.experiment.data.json", experiments_directory(), app_id, project_id, &experiment.id);

        info!("Writing experiment data to file: {}", file_path);

        write_helper(experiment, file_path)
    }

    fn write_audience_list_data(&self, app_id: &str, project_id: &str, audience_list: &AudienceList) -> anyhow::Result<()> {
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

pub struct LocalStore {}

impl ExperimentStore for LocalStore {
    fn write_data<T>(data: &T, file_path: String) -> anyhow::Result<()>
    where
        T: Serialize,
    {
        let file = File::create(&file_path)?;
        let mut writer = BufWriter::new(file);

        // Write the JSON contents to the file.
        serde_json::to_writer(&mut writer, data).with_context(|| format!("Error in writing data to file: {}", file_path))?;

        writer.flush()?;

        Ok(())
    }
}

pub struct S3Store {}

impl ExperimentStore for S3Store {}

fn apps_directory() -> String {
    settings().read().get::<String>("apps_directory").unwrap_or_else(|_| format!("data/apps"))
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
