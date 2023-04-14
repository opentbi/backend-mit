extern crate serde_json;
extern crate grammers_client;

mod util;
mod transferdata;
mod channel;
mod config;
mod rest;
mod rest_util;
mod rest_controllers;

use std::thread;

use grammers_client::{Client, Config, SignInError, types::Media};
use grammers_session::Session;
use grammers_tl_types::enums::MessagesFilter;
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

    unsafe {
        channel::TRANSMITTER = Some(tx.clone());
    }
    if !client.is_authorized().await? {
        let token = client.request_login_code(&cfg.telegram_phone, cfg.app_id, &app_hash).await?;
        let code = util::prompt("Enter verification code: ")?;
        let signed_in = client.sign_in(&token, code.as_str()).await;

        match signed_in {
            Err(SignInError::PasswordRequired(pwd_token)) => {
                let hint = pwd_token.hint().unwrap();
                let prompt_message = format!("Enter the password (hint {}): ", &hint);
                let password = util::prompt(prompt_message.as_str())?;

                client
                    .check_password(pwd_token, password.trim())
                    .await?;
            },
            Ok(_) => (),
            Err(e) => panic!("{}", e),
        }

        client.session().save_to_file("malingit")?;
    }
    println!("Logged in as: {}", client.get_me().await?.username().unwrap());

    drop(app_hash);
    tokio::task::spawn(rest::run_maling_itrest());

    let maling_it_chat = client.resolve_username(cfg.telegram_channel.as_str()).await
        .expect("Chat username resolved").expect("Chat exists");
    while let Some(message) = rx.recv().await {
        match message {
            // WebSearchFile handler
            transferdata::TransferData::WebSearchFile { query, resp_tx } => {
                let mut data: Vec<transferdata::WebSearchFileData> = Vec::new();

                util::find_match_file_query(cfg.cache_file.as_str(), &mut data, &query).await?;
                if data.len() < 1 {
                    let mut msgs = client.search_messages(&maling_it_chat).filter(MessagesFilter::InputMessagesFilterDocument)
                        .query(query.as_str());
                    while let Some(n) = msgs.next().await? {
                        let media = n.media();
                        if !media.is_none() {
                            match media.unwrap() {
                                Media::Document(document) => {
                                    data.push(transferdata::WebSearchFileData {
                                        file_id: document.id().to_string(),
                                        file_mime: document.mime_type().unwrap_or("-").to_string(),
                                        file_name: document.name().to_string(),
                                        file_size: document.size()
                                    });
                                    util::write_cache(cfg.cache_file.as_str(), document.id(), data.last().unwrap()).await?;
                                },
                                _ => ()
                            }
                        }
                    }

                    drop(msgs);
                    println!("[i] Load data from MTProto Telegram for: {}", query);
                } else {
                    println!("[i] Load data from cache for: {}", query);
                }
                resp_tx.send(Some(data)).unwrap();
            },

            transferdata::TransferData::WebDownloadFile { file_id, resp_tx } => {
                let sdata = util::find_file(cfg.cache_file.as_str(), file_id).await?;
                if sdata.file_id.len() < 5 {
                    resp_tx.send(None).unwrap();
                } else {
                    resp_tx.send(Some(transferdata::WebDownloadFile {
                        file_id: file_id,
                        name: sdata.file_name
                    })).unwrap();
                }
            },
        }
    }
    Ok({})
}

#[tokio::main]
async fn main() -> Result {
    let cfg = MitConfig::load();
    mit_main(cfg).await
}
