use std::net::SocketAddr;

use hyper::{server::conn::http1, Request, body::Incoming, service::service_fn, Response};
use tokio::net::TcpListener;

use crate::{rest_util, rest_controllers};

async fn service_mit(request: Request<Incoming>) -> rest_util::ResultServiceFn {
    let res = match request.uri().path() {
        "/find" => {
            rest_controllers::query::rest_query_fn(request).await
        },
        _ => {
            rest_controllers::non_match::rest_non_match_fn()
        }
    };

    Ok(Response::builder().status(res.status).body(res.msg).unwrap())
}

pub async fn run_maling_itrest() {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.expect("TCPListener::bind success");
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(service_mit))
                .await {
                    println!("Error serving connection: {:?}", err);
                }
        });
    }
}