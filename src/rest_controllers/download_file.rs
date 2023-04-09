use std::collections::HashMap;

use http_body_util::Full;
use hyper::{Request, body::{Incoming, Bytes}, StatusCode};

use crate::{rest_util, channel, transferdata};

pub async fn rest_download_file_fn(request: Request<Incoming>) -> rest_util::ResultRestFn {
    let query = if let Some(qu) = request.uri().query() {
        qu
    } else {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("Missing query"),
            is_json: false,
        }
    };
    let params = url::form_urlencoded::parse(query.as_bytes())
        .into_owned()
        .collect::<HashMap<String, String>>();

    let file_id = if let Some(q) = params.get("id") {
        q
    } else {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("Missing id"),
            is_json: false,
        }
    };

    let ok_id = file_id.to_string().parse::<i64>();
    if ok_id.is_err() {
        return rest_util::ResultRestFn {
            status: StatusCode::BAD_REQUEST,
            msg: Full::<Bytes>::from("File ID is invalid"),
            is_json: false,
        }
    }

    let data: Option<transferdata::WebDownloadFile>;
    unsafe {
        let tx = channel::TRANSMITTER.clone();
        let (t, r) = tokio::sync::oneshot::channel();
        tx.unwrap().send(transferdata::TransferData::WebDownloadFile {
            file_id: ok_id.unwrap(),
            resp_tx: t
        }).await.unwrap();

        data = r.await.unwrap();
    }

    let data_json = serde_json::to_string(&data);
    drop(query);
    drop(data);

    if data_json.is_err() {
        return rest_util::ResultRestFn {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            msg: Full::<Bytes>::from(data_json.unwrap_err().to_string()),
            is_json: false,
        }
    }
    return rest_util::ResultRestFn {
        status: StatusCode::OK,
        msg: Full::<Bytes>::from(data_json.unwrap()),
        is_json: true
    }
}