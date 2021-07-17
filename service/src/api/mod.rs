use async_trait::async_trait;
use http::Response;
use hyper::Body;

use crate::server::{HttpResponse, HttpRoute, Service};
use crate::service::AbOptimisationService;

mod common;
mod experiment_runner;
mod experiment_tracking_data;

#[async_trait]
impl Service for AbOptimisationService {
    async fn api_handler<'a>(
        &self,
        body: Body,
        route: &HttpRoute<'a>,
        path: &[&str],
    ) -> anyhow::Result<Response<Body>> {
        match path {
            // sub routes
            ["run"] if matches!(route.method, &http::Method::POST) => {
                experiment_runner::run(self, route, body).await
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
