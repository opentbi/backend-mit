use http_body_util::Full;
use hyper::{StatusCode, body::Bytes, Response};

pub struct ResultRestFn {
    pub status: StatusCode,
    pub msg: Full<Bytes>,
}
pub type ResultServiceFn = Result<Response<Full<Bytes>>, hyper::Error>;
