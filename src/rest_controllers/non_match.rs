use hyper::{body::Incoming, Request};

use crate::rest_util;

pub async fn rest_non_match_fn(request: Request<Incoming>) -> rest_util::ResultRestFn {
    rest_util::build_raw_response("Non-Match", hyper::StatusCode::NOT_FOUND)
}