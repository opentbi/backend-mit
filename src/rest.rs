use std::net::SocketAddr;

use hyper::server::conn::http1;
use tokio::{net::TcpListener, sync::mpsc::Sender};

use crate::transferdata;
use crate::rest_service;


fn service_fn(tx: Sender<transferdata::TransferData>) -> rest_service::MitService {
    rest_service::MitService {
        tx_mpsc: tx,
    }
}

pub async fn run_maling_itrest(tx: Sender<transferdata::TransferData>) {
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = TcpListener::bind(addr).await.expect("TCPListener::bind success");
    loop {
        let (stream, _) = listener.accept().await.unwrap();
        let tx1 = tx.clone();
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(stream, service_fn(tx1))
                .await {
                    println!("Error serving connection: {:?}", err);
                }
        });
    }
}