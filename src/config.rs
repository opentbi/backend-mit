use std::fs;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub app_id: i32,
    pub app_hash: String,
    pub telegram_channel: String,
    pub telegram_phone: String,
    pub cache_file: String
}

impl Config {
    pub fn load() -> Self {
        let contents = fs::read_to_string("config.json")
            .expect("Config filenya terbaca");
        let deserialized: Config = serde_json::from_str(contents.as_str())
            .expect("Isi file config dapat di parse oleh serde_json");

        drop(contents);
        deserialized
    }
}