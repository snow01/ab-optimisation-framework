use super::HttpResponse;
use super::HttpRoute;
use http::{Method, Response};
use hyper::Body;
use std::sync::atomic::Ordering;
use super::app::IN_ROTATION;

fn switch_oor_status() -> anyhow::Result<Response<Body>> {
    let current_value = IN_ROTATION.load(Ordering::Relaxed);
    IN_ROTATION.store(!current_value, Ordering::Relaxed);
    // HttpResponse::json(&!current_value, None)
    get_in_rotation_status()
}

pub fn get_in_rotation_status() -> anyhow::Result<Response<Body>> {
    let in_rotation = IN_ROTATION.load(Ordering::Relaxed);
    if in_rotation {
        const OK: &str = "OK";

        HttpResponse::ok(Body::from(OK), None)
    } else {
        const NOK: &str = "NOK";

        HttpResponse::internal_server_error(Body::from(NOK))
    }
}

const API_PATH: &str = "oor";

pub fn oor_handler(route: &mut HttpRoute<'_>) -> anyhow::Result<Response<Body>> {
    route.metric_path = Some(API_PATH);

    let method = route.method;

    match method {
        &Method::GET => switch_oor_status(),
        &Method::POST => switch_oor_status(),
        _ => HttpResponse::not_found(),
    }
}
