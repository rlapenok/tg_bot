use std::{sync::{Arc, Mutex}, time::Duration, fs::File, io::Write};

use log::info;
use teloxide::{
    requests::{Requester, ResponseResult},
    types::Message,
    Bot,
};

use crate::{build_dispatcher::{create_chema::{AuthDialogue, GlobalDialogue}, commands::Commands}, myauthenticator::MyAuthenticator};

use super::dialogues::Auth;

pub async fn other_msg_hanler_auth(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    bot.send_message(
        chat_id,
        "Братулец, что-то ты не то ввел)) Давайка в ЧВК Вагнер 🇷🇺\n Вводи сча /startauth",
    )
    .await?;
    Ok(())
}

pub async fn init_handler_auth(
    bot: Bot,
    msg: Message,
    cmd: Commands,
    auth: Arc<Mutex<MyAuthenticator>>,
    auth_dialogue: AuthDialogue,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        Commands::StartAuth => {
            let lock = auth.clone().lock().unwrap().exist_key();

            match lock {
                true => {
                    let resp="Братулец, я вижу, что ты тут уже не первый раз\nДавай сходишь в Google Authenticator и возьмешь там код, из ключа,\nкоторый я тебе сгенерил при первом запуске\nВводи свой код из Google Authenticator, а я проверю ты ли это 😈";
                    auth_dialogue.update(Auth::CheckCode).await.unwrap();
                    bot.send_message(chat_id, resp).await?;
                }
                false => {
                    let key = auth.clone().lock().unwrap().create_keys();
                    let bytes=key.as_bytes();
                    File::create("key.txt").unwrap().write_all(bytes).unwrap();
                    info!("Create key.txt nad put key");
                    let info="Братулец, я так понимаю, что ты здесь первый раз => я тебе сгенерил ключ для приложения Google Authenticator 🔑\n Давайка вставим его в приложение \n";
                    let resp = format!("{}Твой 🔑: {}", info, key);
                    bot.send_message(chat_id, resp).await?;
                    auth_dialogue.update(Auth::CheckCode).await.unwrap();
                    tokio::time::sleep(Duration::from_secs(10)).await;
                    let resp_2="Итак, ты уже должен был 🔑 Google Authenticator 🔑\nТеперь возьми оттуда 6-ти значный код и вставь в меня 😝";
                    bot.send_message(chat_id, resp_2).await?;
                }
            };
        }
        _ => {}
    }
    Ok(())
}

pub async fn check_code_handler_auth(
    bot: Bot,
    msg: Message,
    auth: Arc<Mutex<MyAuthenticator>>,
    dialogue_auth: AuthDialogue,
    dialogue_global:GlobalDialogue
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let code = msg.text().unwrap().trim();
    match code.parse::<usize>() {
        Ok(_) => {
            let auth = auth.clone().lock().unwrap().verify_code(code);
            match auth {
                true => {
                    let resp="Братулец, все 👌 => можешь дальше не переживать,\nчто еще кто-то будет пользоваться твоими приватниками";
                    dialogue_auth.exit().await.unwrap();
                    dialogue_global.update(super::dialogues::Global::GetWallet).await.unwrap();
                    bot.send_message(chat_id, resp).await?;

                }
                false => {
                    let resp="Братулец, что-то пошло не так 🐵 => ты хочешь меня где-то наебать,\nПошел нахуй, козел,ебаный 💐\nДава вводи ключ заново";
                    bot.send_message(chat_id, resp).await?;
                }
            }
        }
        Err(_) => {
            bot.send_message(chat_id, "Братулец, ты ввел не число\nПопробуй еще раз")
                .await?;
        }
    }

    Ok(())
}
