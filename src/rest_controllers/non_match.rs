use http_body_util::Full;
use hyper::{body::Bytes, StatusCode};

use crate::rest_util;

pub fn rest_non_match_fn() -> rest_util::ResultRestFn {
    return rest_util::ResultRestFn {
        status: StatusCode::NOT_FOUND,
        msg: Full::<Bytes>::from("Non-Match"),
        content_type: "text/html".to_string()
    }
}