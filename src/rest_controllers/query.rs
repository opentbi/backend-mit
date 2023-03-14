use std::collections::HashMap;

use http_body_util::Full;
use hyper::{Request, body::{Incoming, Bytes}, StatusCode};

use crate::{rest_util, transferdata, channel};

pub async fn rest_query_fn(request: Request<Incoming>) -> rest_util::ResultRestFn {
    let query = if let Some(qu) = request.uri().query() {
        qu
    } else {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("Missing query")
        }
    };
    let params = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let query_result = if let Some(q) = params.get("query") {
        q
    } else {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("Missing query")
        }
    };

    if query_result.len() < 4 {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("Invalid query")
        }
    }

    unsafe {
        let tx = channel::TRANSMITTER.clone();
        tx.unwrap().send(transferdata::TransferData {
            t: transferdata::TransferDataType::WebSearchFile,
            search_file_query: Some(query_result.to_string()),
            search_file_id: Some(String::new()),
        }).await.unwrap();
    }

    drop(query);

    return rest_util::ResultRestFn {
        status: StatusCode::OK,
        msg: Full::<Bytes>::from("Sent")
    }
}