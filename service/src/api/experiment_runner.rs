use serde_json::Value as JsonValue;
use serde::{Serialize, Deserialize};
use crate::application::AbOptimisationApplication;
use crate::server::{HttpRoute, HttpRequest, HttpResponse};
use hyper::Body;

pub struct ExperimentRunner {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub user_id: String,
    pub project_id: String,
    pub ctx: JsonValue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub project_id: String,
    pub active_experiments: Vec<ActiveExperiment>,

    #[serde(with = "humantime_serde")]
    pub time_taken: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveExperiment {
    pub experiment: String,
    pub variation: String,
}

pub async fn run(app: &AbOptimisationApplication, route: &HttpRoute<'_>, body: Body) -> anyhow::Result<http::Response<Body>>
{
    let data = HttpRequest::body(route, body).await?;
    // let data_format = get_data_format(route)?;
    //
    // let req_format = data_format.clone();
    //
    // app.node()
    //     .batch_api_request(data, data_format)
    //     //.map_err(|e| HttpError::from(e))
    //     .and_then(move |data| match req_format {
    //         DataFormat::AVRO => batch::avro_response(data).and_then(|data| Ok(HttpResponse::Ok().body(data))),
    //         DataFormat::JSON => Ok(HttpResponse::Ok().json(batch::json_response(data))),
    //         DataFormat::UNKNOWN => Ok(HttpResponse::BadRequest().body("UNKNOWN data format provided in request")),
    //     })
    //     .map_err(|e| HttpError::from(e))

    let response_body = Response {
        project_id: "".to_string(),
        active_experiments: vec![],
        time_taken: Default::default(),
    };

    HttpResponse::json(&response_body, Some("gzip".as_ref()))
}

impl ExperimentRunner {}