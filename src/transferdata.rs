#[derive(Debug, Clone)]
pub enum TransferDataType {
    WebSearchFile
}
#[derive(Debug, Clone)]
pub struct TransferData {
    pub t: TransferDataType,
    pub search_file_query: Option<String>,
    pub search_file_id: Option<String>
}
