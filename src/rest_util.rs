use http_body_util::Full;
use hyper::{Response, body::Bytes};

pub type ResultRestFn = Result<Response<Full<Bytes>>, hyper::Error>;
pub fn build_raw_response(text: &'static str, status: hyper::StatusCode) -> ResultRestFn {
    Ok(Response::builder().status(status).body(Full::from(Bytes::from(text))).unwrap())
}
