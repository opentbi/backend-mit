use http_body_util::Full;
use hyper::{StatusCode, body::Bytes, Response};

pub struct ResultRestFn {
    pub status: StatusCode,
    pub msg: Full<Bytes>,
    pub is_json: bool
}
pub type ResultServiceFn = Result<Response<Full<Bytes>>, hyper::Error>;
