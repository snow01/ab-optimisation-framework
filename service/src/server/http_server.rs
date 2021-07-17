use std::mem;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Instant;

use anyhow::Context;
use http::{HeaderValue, Method, Request, Response};
use hyper::server::conn::AddrStream;
use hyper::service::{make_service_fn, service_fn};
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};

use crate::server::{Service, ServiceBuilder};

use super::access_logger::ACCESS_LOGGER;
use super::health_check::{get_in_rotation_status, oor_handler};
use super::http_response::HttpResponse;
// use super::ACCESS_LOGGER;
use super::HttpRoute;

fn index(route: &HttpRoute<'_>) -> anyhow::Result<Response<Body>> {
    let body = Body::from("Hello!");
    HttpResponse::ok(route, body)
}

async fn shutdown_signal() {
    // Wait for the CTRL+C signal
    debug!("Installing shutdown signal");

    tokio::signal::ctrl_c()
        .await
        .expect("failed to install CTRL+C signal handler");

    warn!("Received shutdown signal");
}

// TODO: payload limit - json_payload_limit_conf()
async fn route_handler<App>(
    mut req: Request<Body>,
    remote_addr: std::net::SocketAddr,
    app: Arc<App>,
) -> anyhow::Result<Response<Body>>
where
    App: 'static + Service,
{
    let req_time = chrono::Local::now();
    let req_instant = Instant::now();

    let req_body = mem::replace(req.body_mut(), Body::empty());
    let route = HttpRoute::new(&req, req_time, req_instant, remote_addr);

    // let body_buf;
    // if matches!(route.method, &Method::POST) || matches!(route.method, &Method::PUT) || matches!(route.method, &Method::PATCH) {
    //     body_buf = HttpRequest::body(&route, req_body).await?;
    // } else {
    //     // optimise this further
    //     body_buf = HttpRequest::body(&route, Body::empty()).await?;
    // }

    let parts: Vec<_> = route
        .path
        .split("/")
        .filter(|part| !part.is_empty())
        .collect();

    let response = match &parts[..] {
        [] if matches!(route.method, &Method::GET) => index(&route),
        ["oor"] => oor_handler(&route),
        ["status"] if matches!(route.method, &Method::GET) => get_in_rotation_status(&route),
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
        ["api", rest @ ..] => app.api_handler(req_body, &route, rest).await,
        _ => HttpResponse::not_found(route.path),
    };

    let response = match response {
        Ok(mut response) => {
            let time_taken = format!("{}", humantime::Duration::from(req_instant.elapsed()));
            let time_taken_header = HeaderValue::from_str(&time_taken)?;
            response
                .headers_mut()
                .append("X-time-taken", time_taken_header);
            Ok(response)
        }
        Err(err) => HttpResponse::internal_server_error(err),
    };

    // log & metrics
    ACCESS_LOGGER.log_access(&route, &response);

    response
}

pub async fn start_http_server<App, AppBuilder>(
    addr: &str,
    app_builder: AppBuilder,
) -> anyhow::Result<()>
where
    App: 'static + Service,
    AppBuilder: 'static + ServiceBuilder<App>,
{
    info!("Starting server at addr: {}", addr);

    let addr = addr
        .parse::<SocketAddr>()
        .with_context(|| format!("Parsing node addr '{}' as SocketAddr", addr))?;

    let app = app_builder
        .build()
        .with_context(|| "Error in building app")?;

    let app = Arc::new(app);

    let make_svc = make_service_fn(move |transport: &AddrStream| {
        // TODO: log new connection
        let remote_addr = transport.remote_addr();
        let app = app.clone();

        async move {
            Ok::<_, anyhow::Error>(service_fn(move |req| {
                // Clone again to ensure that client outlives this closure.
                route_handler(req, remote_addr, app.clone())
            }))
        }
    });

    let server = hyper::Server::try_bind(&addr)
        .with_context(|| "Error in binding to address")?
        .http1_keepalive(true)
        .http1_preserve_header_case(true)
        .http1_title_case_headers(true)
        .serve(make_svc);

    let graceful = server.with_graceful_shutdown(shutdown_signal());

    info!("Started server");

    // Run this server for... forever!
    graceful.await.with_context(|| "Error in starting server")
}
