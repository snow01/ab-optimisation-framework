use super::commons::{BR_CONTENT_ENCODING, DEFLATE_CONTENT_ENCODING, GZIP_CONTENT_ENCODING};
use crate::server::commons::get_hostname_header;
use anyhow::Context;
use futures::{TryStreamExt, Stream};
use http::{header, Response};
use http::{HeaderValue, StatusCode};
use hyper::Body;
use serde::Serialize;

pub struct HttpResponse;

lazy_static! {
    static ref BR_HEADER_VALUE: HeaderValue = HeaderValue::from_static("br");
    static ref DEFLATE_HEADER_VALUE: HeaderValue = HeaderValue::from_static("deflate");
    static ref GZIP_HEADER_VALUE: HeaderValue = HeaderValue::from_static("gzip");
}

impl HttpResponse {
    pub fn ok(body: Body, content_encoding: Option<&[u8]>) -> anyhow::Result<Response<Body>> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::HOST, get_hostname_header().clone())
            .body(body)
            .with_context(|| "Error in building HttpResponse")?;

        Ok(Self::compress_response(response, content_encoding))
    }

    pub fn string(body: String, content_encoding: Option<&[u8]>) -> anyhow::Result<Response<Body>> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::HOST, get_hostname_header().clone())
            .body(Body::from(body))
            .with_context(|| "Error in building HttpResponse")?;

        Ok(Self::compress_response(response, content_encoding))
    }

    pub fn str(body: &'static str, content_encoding: Option<&[u8]>) -> anyhow::Result<Response<Body>> {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::HOST, get_hostname_header().clone())
            .body(Body::from(body))
            .with_context(|| "Error in building HttpResponse")?;

        Ok(Self::compress_response(response, content_encoding))
    }

    pub fn json<S>(body: &S, content_encoding: Option<&[u8]>) -> anyhow::Result<Response<Body>>
    where
        S: Serialize,
    {
        let body = serde_json::to_vec(body).with_context(|| "Error in serialising")?;
        let body = Body::from(body);

        let response = Response::builder()
            .status(StatusCode::OK)
            .header(header::CONTENT_TYPE, "application/json")
            .header(header::HOST, get_hostname_header().clone())
            .body(body)
            .with_context(|| "Error in building HttpResponse")?;

        Ok(Self::compress_response(response, content_encoding))
    }

    pub fn compress_response(mut response: Response<Body>, content_encoding: Option<&[u8]>) -> Response<Body> {
        use std::io::{Error as IOError, ErrorKind as IOErrorKind};

        // compress as needed
        if let Some(accept_encoding) = content_encoding {
            match accept_encoding {
                BR_CONTENT_ENCODING => {
                    response.headers_mut().insert(header::CONTENT_ENCODING, BR_HEADER_VALUE.clone());
                    response = response
                        .map(|body| Body::wrap_stream(brotli_encode(body.map_err(|_| IOError::from(IOErrorKind::InvalidData)))));
                }
                DEFLATE_CONTENT_ENCODING => {
                    response
                        .headers_mut()
                        .insert(header::CONTENT_ENCODING, DEFLATE_HEADER_VALUE.clone());
                    response = response
                        .map(|body| Body::wrap_stream(deflate_encode(body.map_err(|_| IOError::from(IOErrorKind::InvalidData)))));
                }
                GZIP_CONTENT_ENCODING => {
                    response.headers_mut().insert(header::CONTENT_ENCODING, GZIP_HEADER_VALUE.clone());
                    response =
                        response.map(|body| Body::wrap_stream(gzip_encode(body.map_err(|_| IOError::from(IOErrorKind::InvalidData)))));
                }
                _ => {
                    // do nothing
                }
            }
        }

        response
    }

    pub fn internal_server_error(body: Body) -> anyhow::Result<Response<Body>> {
        Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header(header::HOST, get_hostname_header().clone())
            .body(body)
            .with_context(|| "Error in building HttpResponse")
    }

    pub fn not_found() -> anyhow::Result<Response<Body>> {
        const NOTFOUND: &str = "Not Found";
        let body = Body::from(NOTFOUND);

        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .header(header::HOST, get_hostname_header().clone())
            .body(body)
            .with_context(|| "Error in building HttpResponse")
    }
}

fn gzip_encode(
    input: impl Stream<Item=std::io::Result<bytes::Bytes>>,
) -> impl Stream<Item=std::io::Result<bytes::Bytes>> {
    tokio_util::io::ReaderStream::new(
        async_compression::tokio::bufread::GzipEncoder::new(
            tokio_util::io::StreamReader::new(input),
        ),
    )
}

fn brotli_encode(
    input: impl Stream<Item=std::io::Result<bytes::Bytes>>,
) -> impl Stream<Item=std::io::Result<bytes::Bytes>> {
    tokio_util::io::ReaderStream::new(
        async_compression::tokio::bufread::BrotliEncoder::new(
            tokio_util::io::StreamReader::new(input),
        ),
    )
}

fn deflate_encode(
    input: impl Stream<Item=std::io::Result<bytes::Bytes>>,
) -> impl Stream<Item=std::io::Result<bytes::Bytes>> {
    tokio_util::io::ReaderStream::new(
        async_compression::tokio::bufread::DeflateEncoder::new(
            tokio_util::io::StreamReader::new(input),
        ),
    )
}
