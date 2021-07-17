use std::fs::File;
use std::io::BufReader;

use anyhow::Context;
use crossbeam_epoch as epoch;
use crossbeam_skiplist::SkipList;
use walkdir::WalkDir;

use crate::config::settings;
use crate::core::{App, Project};
use crate::server::ServiceBuilder;

// #[derive(Clone)]
pub struct AbOptimisationService {
    pub apps: SkipList<String, App>,
}

pub struct AbOptimisationServiceBuilder {}

impl ServiceBuilder<AbOptimisationService> for AbOptimisationServiceBuilder {
    fn build(self) -> anyhow::Result<AbOptimisationService> {
        let service = AbOptimisationService {
            apps: SkipList::new(epoch::default_collector().clone()),
        };

        load_all_apps(&service)?;
        load_all_projects(&service)?;

        Ok(service)
    }
}

impl AbOptimisationService {
    pub fn add_app(&self, app: App) {
        let guard = &epoch::pin();
        self.apps.insert(app.id.to_string(), app, guard);
    }

    pub fn add_project(&self, project: Project) -> anyhow::Result<()> {
        let guard = &epoch::pin();
        match self.apps.get(&project.app, guard) {
            None => {}
            Some(entry) => {
                entry
                    .value()
                    .projects
                    .insert(project.id.to_string(), project, guard);
            }
        }

        Ok(())
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

fn load_all_apps(service: &AbOptimisationService) -> anyhow::Result<()> {
    for entry in WalkDir::new(apps_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with("-app-data.json") {
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let app: App = serde_json::from_reader(reader).with_context(|| {
                format!(
                    "Error in parsing app from file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
            service.add_app(app);
        }
    }

    Ok(())
}

fn load_all_projects(service: &AbOptimisationService) -> anyhow::Result<()> {
    for entry in WalkDir::new(projects_directory())
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with("-project-data.json") {
            let file = File::open(entry.path())?;
            let reader = BufReader::new(file);

            // Read the JSON contents of the file as an instance of `User`.
            let project: Project = serde_json::from_reader(reader).with_context(|| {
                format!(
                    "Error in parsing project from file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
            service.add_project(project).with_context(|| {
                format!(
                    "Error in adding project for file: {}",
                    entry.path().to_string_lossy()
                )
            })?;
        }
    }

    Ok(())
}
