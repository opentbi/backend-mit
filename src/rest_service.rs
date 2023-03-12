use std::{pin::Pin, future::Future};

use http_body_util::Full;
use hyper::{service::Service, Request, body::{Incoming, Bytes}, Response};
use tokio::sync::mpsc::Sender;

use crate::{transferdata, rest_util::{ResultRestFn}, rest_controllers};

#[derive(Clone)]
pub struct MitService {
    pub tx_mpsc: Sender<transferdata::TransferData>,
}

impl Service<Request<Incoming>> for MitService {
    type Response = Response<Full<Bytes>>;
    type Error = hyper::Error;
    type Future = Pin<Box<dyn Future<Output = ResultRestFn> + Send>>;

    fn call(&mut self, req: Request<Incoming>) -> Self::Future {
        let res = match req.uri().path() {
            "/find" => {
                rest_controllers::query::rest_query_fn(req, self.tx_mpsc.clone())
            },
            _ => {
                // TODO: fix this fucking bug
                rest_controllers::non_match::rest_non_match_fn(req)
            }
        };

        Box::pin(async { res })
    }
}
