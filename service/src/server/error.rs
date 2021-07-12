// use std::fmt;
// use std::fmt::Display;

// use thiserror::Error;
//
// #[derive(Error, Debug)]
// pub struct HttpError {
//     source: anyhow::Error,
// }
//
// impl ResponseError for HttpError {
//     //    fn render_response(&self) -> actix_web::HttpResponse {
//     //        let mut resp = self.error_response();
//     //        let mut buf = BytesMut::new();
//     //        let _ = write!(buf, "{:?}", self);
//     //        resp.headers_mut()
//     //            .insert(header::CONTENT_TYPE, header::HeaderValue::from_static("text/plain"));
//     //        resp.set_body(Body::from(buf))
//     //    }
// }
//
// impl From<anyhow::Error> for HttpError {
//     fn from(err: anyhow::Error) -> Self {
//         HttpError { source: err }
//     }
// }
//
// impl Display for HttpError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
//         write!(f, "Http Request Error. Cause: {}", self.source)
//     }
// }
