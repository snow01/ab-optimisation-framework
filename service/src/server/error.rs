use http::Response;
use hyper::Body;
#[allow(unused_imports)]
use log::{debug, error, info, warn};
use thiserror::Error;

use crate::server::HttpResponse;

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Internal Server Error: {0}")]
    InternalServerError(anyhow::Error),

    #[error("Not Found Error: {0}")]
    NotFound(String),

    #[error("Forbidden Error: {0}")]
    Forbidden(String),

    #[error("Bad Request Error: {0}")]
    BadRequest(#[from] anyhow::Error),

    #[error("Not Content: {0}")]
    NoContent(String),
}

impl Into<anyhow::Result<http::Response<Body>>> for HttpError {
    fn into(self) -> anyhow::Result<Response<Body>> {
        match self {
            HttpError::InternalServerError(error) => HttpResponse::internal_server_error(error),
            HttpError::NotFound(reason) => HttpResponse::not_found(&reason),
            HttpError::Forbidden(reason) => HttpResponse::forbidden(&reason),
            HttpError::BadRequest(error) => HttpResponse::bad_request(error),
            HttpError::NoContent(reason) => HttpResponse::no_content(&reason),
        }
    }
}
