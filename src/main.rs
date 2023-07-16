use log::info;
use std::{
    env,
    sync::{Arc, Mutex},
};
use teloxide::{dispatching::dialogue::InMemStorage, dptree::deps, prelude::Dispatcher, Bot};
use tokio::spawn;

pub mod auto_generate;
pub mod build_dispatcher;
pub mod client;
pub mod myauthenticator;

use crate::build_dispatcher::dialogues::dialogues::{Auth, Global, SelectWallets};
use build_dispatcher::create_chema::create_schema;

fn init() {
    dotenv::dotenv().unwrap();
    env_logger::init();
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    init();
    info!("Start tg_bot...");
    let token = env::var("TOKEN")?;
    let host = env::var("PORT")?;
    //Создание аунтентификатора
    let auth = myauthenticator::MyAuthenticator::new();
    let client = client::client::Client::new(host).await?;
    //Создание экземпляра бота
    let bot = Bot::new(token);
    //Создание схемы обработки диалогов в боте
    let schema = create_schema();
    //Посторение на основании схемы бота
    Dispatcher::builder(bot, schema)
        .dependencies(deps![
            Arc::new(Mutex::new(auth)),
            client,
            InMemStorage::<Global>::new(),
            InMemStorage::<Auth>::new(),
            InMemStorage::<SelectWallets>::new()
        ])
        .build()
        .dispatch()
        .await;

    //Dead tg_bot ctrl+c
    spawn(async { tokio::signal::ctrl_c().await.unwrap() }).await?;
    Ok(())
}
