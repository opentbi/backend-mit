extern crate serde_json;
extern crate grammers_client;

mod transferdata;
mod config;
mod rest_service;
mod rest;
mod rest_util;
mod rest_controllers;

use grammers_client::{Client, Config};
use grammers_session::Session;
use tokio::sync::mpsc;
use crate::config::Config as MitConfig;

type Result = std::result::Result<(), Box<dyn std::error::Error>>;

fn takes_hash_own(hash: String) -> String {
    let mut s1 = String::new();
    s1.clone_from(&hash);

    s1
}

async fn mit_main(cfg: MitConfig) -> Result {
    let app_hash = takes_hash_own(cfg.app_hash);
    let client =  Client::connect(Config {
        api_id: cfg.app_id,
        api_hash: app_hash.to_string(),
        session: Session::load_file_or_create("malingit")?,
        params: Default::default()
    }).await?;
    let (tx, mut rx) = mpsc::channel::<transferdata::TransferData>(32);

    if !client.is_authorized().await? {
        client.bot_sign_in(&cfg.telegram_bot_token, cfg.app_id, &app_hash).await?;
        client.session().save_to_file("malingit")?;
    }
    println!("Logged in as: {}", client.get_me().await?.username().unwrap());

    drop(app_hash);

    tokio::task::spawn(rest::run_maling_itrest(tx));

    while let Some(message) = rx.recv().await {
        println!("{:?}", message);
    }
    Ok({})
}

#[tokio::main]
async fn main() -> Result {
    let cfg = MitConfig::load();
    mit_main(cfg).await
}
