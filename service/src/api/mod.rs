use async_trait::async_trait;
use http::Response;
use hyper::Body;

use crate::server::{ApiError, HttpResponse, HttpRoute, Service};
use crate::service::AbOptimisationService;

mod common;
mod experiment_runner;
mod experiment_tracking_data;

#[async_trait]
impl Service for AbOptimisationService {
    async fn api_handler<'a>(
        &'a self,
        body: Body,
        route: &HttpRoute<'a>,
        path: &[&str],
    ) -> Result<Response<Body>, ApiError> {
        match path {
            // sub routes
            ["run"] if matches!(route.method, &http::Method::POST) => self.run(route, body).await,

            ["apps", app_id] if matches!(route.method, &http::Method::GET) => {
                self.get_app(route, app_id).await
            }

            ["apps", app_id] if matches!(route.method, &http::Method::POST) => {
                self.update_app(route, app_id, body).await
            }

            ["apps"] if matches!(route.method, &http::Method::POST) => {
                self.add_app(route, body).await
            }

            ["apps"] if matches!(route.method, &http::Method::GET) => self.list_apps(route).await,

            ["projects", app_id, project_id] if matches!(route.method, &http::Method::GET) => {
                self.get_project(route, app_id, project_id).await
            }

            ["projects", app_id, project_id] if matches!(route.method, &http::Method::POST) => {
                self.update_project(route, app_id, project_id, body).await
            }

            ["projects", app_id] if matches!(route.method, &http::Method::POST) => {
                self.add_project(route, app_id, body).await
            }

            ["projects", app_id] if matches!(route.method, &http::Method::GET) => {
                self.list_projects(route, app_id).await
            }

            ["audience-lists", app_id, project_id, list_id]
                if matches!(route.method, &http::Method::GET) =>
            {
                self.get_audience_list(route, app_id, project_id, list_id)
                    .await
            }

            ["audience-lists", app_id, project_id, list_id]
                if matches!(route.method, &http::Method::POST) =>
            {
                self.update_audience_list(route, app_id, project_id, list_id, body)
                    .await
            }

            ["audience-lists", app_id, project_id]
                if matches!(route.method, &http::Method::POST) =>
            {
                self.add_audience_list(route, app_id, project_id, body)
                    .await
            }

            ["audience-lists", app_id, project_id]
                if matches!(route.method, &http::Method::GET) =>
            {
                self.list_audience_lists(route, app_id, project_id).await
            }

            ["experiments", app_id, project_id, experiment_id]
                if matches!(route.method, &http::Method::GET) =>
            {
                self.get_experiment(route, app_id, project_id, experiment_id)
                    .await
            }

            ["experiments", app_id, project_id, experiment_id]
                if matches!(route.method, &http::Method::POST) =>
            {
                self.update_experiment(route, app_id, project_id, experiment_id, body)
                    .await
            }

            ["experiments", app_id, project_id] if matches!(route.method, &http::Method::POST) => {
                self.add_experiment(route, app_id, project_id, body).await
            }

            ["experiments", app_id, project_id] if matches!(route.method, &http::Method::GET) => {
                self.list_experiments(route, app_id, project_id).await
            }

            // ["schema", rest @ ..] => HttpResponse::not_found(),
            //
            // ["topology", rest @ ..] => HttpResponse::not_found(),
            //
            // ["data", rest @ ..] => data_crud_handler(&mut route, body, rest, app).await,
            //
            // ["related-data", rest @ .., "search"] if matches!(route.method, &Method::POST) => {}
            //
            // ["search", rest @ ..] if matches!(route.method, &Method::POST) => search_handler(&mut route, body, rest, app).await,
            //
            // ["job-manager", rest @ ..] => HttpResponse::not_found(),
            _ => HttpResponse::not_found(route.path),
        }
    }
}
