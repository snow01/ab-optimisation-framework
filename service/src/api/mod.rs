mod experiment_runner;

use crate::server::{Application, HttpRoute, HttpResponse};
use hyper::Body;
use http::Response;
use async_trait::async_trait;
use crate::application::AbOptimisationApplication;

#[async_trait]
impl Application for AbOptimisationApplication {
    async fn api_handler<'a>(&self, body: Body, route: &mut HttpRoute<'a>, path: &[&str]) -> anyhow::Result<Response<Body>>
    {
        match path {
            // sub routes
            ["run"] if matches!(route.method, &http::Method::POST) => experiment_runner::run(self, route, body).await,

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

            _ => HttpResponse::not_found(),
        }
    }
}