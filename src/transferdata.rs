use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSearchFileData {
    pub file_id: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_mime: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebDownloadFile {
    pub file_id: i64,
    pub name: String
}

#[derive(Debug)]
pub enum TransferData {
    WebSearchFile {
        query: String,
        resp_tx: tokio::sync::oneshot::Sender<Option<Vec<WebSearchFileData>>>
    },
    WebDownloadFile {
        file_id: i64,
        resp_tx: tokio::sync::oneshot::Sender<Option<WebDownloadFile>>
    }
}
