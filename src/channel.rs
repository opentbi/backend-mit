use tokio::sync::mpsc;

use crate::transferdata;

pub static mut TRANSMITTER: Option<mpsc::Sender<transferdata::TransferData>> = None;

