use std::sync::atomic::AtomicBool;
use hyper::Body;
use crate::server::HttpRoute;
use http::Response;
use async_trait::async_trait;

lazy_static! {
    pub static ref IN_ROTATION: AtomicBool = AtomicBool::new(true);
}

pub trait ApplicationBuilder<T: Application> {
    fn build(self) -> anyhow::Result<T>;
}

#[async_trait]
pub trait Application: Send + Sync + Clone {
    async fn api_handler<'a>(&self, body: Body, route: &mut HttpRoute<'a>, path: &[&str]) -> anyhow::Result<Response<Body>>;
}
