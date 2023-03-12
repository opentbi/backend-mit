use std::collections::HashMap;

use hyper::{Request, body::{Incoming}};
use tokio::sync::mpsc::Sender;

use crate::{rest_util, transferdata};

pub async fn rest_query_fn(request: Request<Incoming>, tx: Sender<transferdata::TransferData>) -> rest_util::ResultRestFn {
    let query = if let Some(qu) = request.uri().query() {
        qu
    } else {
        return rest_util::build_raw_response("Couldn't get URI", hyper::StatusCode::BAD_REQUEST);
    };
    let params = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let query_result = if let Some(q) = params.get("query") {
        q
    } else {
        return rest_util::build_raw_response("Missing query", hyper::StatusCode::BAD_REQUEST);
    };

    if query_result.len() < 4 {
        return rest_util::build_raw_response("Invalid query", hyper::StatusCode::BAD_REQUEST);
    }

    tx.send(transferdata::TransferData {
        t: transferdata::TransferDataType::WebSearchFile,
        search_file_query: Some(query_result.to_string()),
        search_file_id: Some(String::new()),
    }).await.unwrap();

    drop(query);

    rest_util::build_raw_response("p", hyper::StatusCode::OK)
}