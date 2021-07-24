use std::sync::atomic::AtomicBool;

use async_trait::async_trait;
use http::Response;
use hyper::Body;

use crate::server::HttpRoute;

lazy_static! {
    pub static ref IN_ROTATION: AtomicBool = AtomicBool::new(true);
}

pub trait ServiceBuilder<T: Service> {
    fn build(self) -> anyhow::Result<T>;
}

#[async_trait]
pub trait Service: Send + Sync {
    async fn api_handler<'a>(
        &'a self,
        body: Body,
        route: &HttpRoute<'a>,
        path: &[&str],
    ) -> anyhow::Result<Response<Body>>;
}
