use super::health_check::{get_in_rotation_status, oor_handler};
use super::http_response::HttpResponse;
// use super::ACCESS_LOGGER;
use super::HttpRoute;
use anyhow::Context;
use http::{Method, Request, Response};
use hyper::service::{make_service_fn, service_fn};
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use std::mem;
use std::sync::Arc;
use std::time::Instant;
use hyper::Body;
use std::net::SocketAddr;
use hyper::server::conn::AddrStream;
use crate::server::{ApplicationBuilder, Application};

fn index() -> anyhow::Result<Response<Body>> {
    let body = Body::from("Hello!");
    HttpResponse::ok(body, None)
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    debug!("Installing shutdown signal");

    tokio::signal::ctrl_c().await.expect("failed to install CTRL+C signal handler");

    warn!("Received shutdown signal");
}

// TODO: payload limit - json_payload_limit_conf()
async fn route_handler<App>(mut req: Request<Body>, remote_addr: std::net::SocketAddr, app: Arc<App>) -> anyhow::Result<Response<Body>>
    where
        App: 'static + Application,
{
    let req_time = chrono::Local::now();
    let req_instant = Instant::now();

    let body = mem::replace(req.body_mut(), Body::empty());

    let mut route = HttpRoute::new(&req, req_time, req_instant, remote_addr);

    let parts: Vec<_> = route.path.split("/").filter(|part| !part.is_empty()).collect();

    let response = match &parts[..] {
        [] if matches!(route.method, &Method::GET) => index(),
        ["oor"] => oor_handler(&mut route),
        ["status"] if matches!(route.method, &Method::GET) => get_in_rotation_status(),
        // ["metrics", rest @ ..] if matches!(route.method, &Method::GET) => match rest {
        //     // sub routes
        //     ["service", "prometheus"] => ACCESS_LOGGER.get_api_metrics_for_prometheus(&route).await,
        //
        //     ["service"] => ACCESS_LOGGER.get_api_metrics_as_json(&route).await,
        //     _ => {
        //         // TODO: other routes...
        //         HttpResponse::not_found()
        //     }
        // },
        ["api", rest @ ..] => app.api_handler(body, &mut route, rest).await,
        _ => HttpResponse::not_found(),
    };

    let response = match response {
        Ok(response) => Ok(response),
        Err(err) => HttpResponse::internal_server_error(Body::from(format!("Error in serving request ==> {:?}", err))),
    };

    // log & metrics
    // ACCESS_LOGGER.log_access(&route, &response);

    response
}

pub async fn start_http_server<App, AppBuilder>(addr: &str, app_builder: AppBuilder) -> anyhow::Result<()>
    where
        App: 'static + Application,
        AppBuilder: 'static + ApplicationBuilder<App>
{
    info!("Starting server at addr: {}", addr);

    let addr = addr
        .parse::<SocketAddr>()
        .with_context(|| format!("Parsing node addr '{}' as SocketAddr", addr))?;

    let app = app_builder.build().with_context(|| "Error in building app")?;

    let app = Arc::new(app);

    let make_svc = make_service_fn(move |transport: &AddrStream| {
        // TODO: log new connection
        let remote_addr = transport.remote_addr();
        let app = app.clone();

        async move {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                route_handler(req, remote_addr.clone(), app.clone())
            }))
        }
    });

    let server = hyper::Server::try_bind(&addr).with_context(|| "Error in binding to address")?
        .http1_keepalive(true)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    info!("Started server");

    // Run this server for... forever!
    graceful.await.with_context(|| "Error in starting server")
}
