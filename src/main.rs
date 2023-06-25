use crate::build_dispatcher::dialogues::dialogues::Auth;
use std::{
    env,
    sync::{Arc, Mutex},
};

use log::info;
use teloxide::{dispatching::dialogue::InMemStorage, dptree::deps, prelude::Dispatcher, Bot};

pub mod auto_generate;
pub mod build_dispatcher;
pub mod client;
pub mod myauthenticator;

use crate::build_dispatcher::dialogues::dialogues::Global;
use build_dispatcher::create_chema::create_schema;
use tokio::spawn;
async fn init() {
    dotenv::dotenv().unwrap();
    env_logger::init();
}

#[tokio::main]
async fn main() {
    init().await;
    info!("Start tg_bot...");
    let token = env::var("TOKEN").unwrap();
    let auth = myauthenticator::MyAuthenticator::new();
    //let  client=client::client::Client::new("http://[::1]:8080").await;
    let bot = Bot::new(token);
    let schema = create_schema();
    Dispatcher::builder(bot, schema)
        .dependencies(deps![
            Arc::new(Mutex::new(auth)),
            InMemStorage::<Global>::new(),
            InMemStorage::<Auth>::new()
        ])
        .build()
        .dispatch()
        .await;

    //Dead ctrl+c
    spawn(async { tokio::signal::ctrl_c().await.unwrap() })
        .await
        .unwrap();
}
