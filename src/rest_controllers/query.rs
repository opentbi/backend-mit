use std::collections::HashMap;

use http_body_util::Full;
use hyper::{Request, body::{Incoming, Bytes}, StatusCode};
use serde_json::json;

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

    let data: serde_json::Value;

    unsafe {
        let tx = channel::TRANSMITTER.clone();
        let (t, r) = tokio::sync::oneshot::channel();
        tx.unwrap().send(transferdata::TransferData::WebSearchFile {
            query: query_result.to_string(),
            resp_tx: t
        }).await.unwrap();

        data = json!(r.await.unwrap().unwrap());
    }
    let data_json = serde_json::to_string(&data);
    drop(query);
    drop(data);

    if data_json.is_err() {
        return rest_util::ResultRestFn {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: Full::<Bytes>::from(data_json.unwrap_err().to_string())
        }
    }
    return rest_util::ResultRestFn {
        status: StatusCode::OK,
        msg: Full::<Bytes>::from(data_json.unwrap())
    }
}