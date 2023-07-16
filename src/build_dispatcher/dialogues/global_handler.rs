use teloxide::{
    requests::{Requester, ResponseResult},
    types::Message,
    utils::command::BotCommands,
    Bot,
};

use crate::build_dispatcher::{commands::Commands, create_chema::GlobalDialogue};

use super::dialogues::{Auth, Global};

pub async fn other_msg_hanler_global(
    bot: Bot,
    msg: Message,
    dialogue: GlobalDialogue,
) -> ResponseResult<()> {
    let descr = Commands::descriptions()
        .to_string()
        .split_once("\n\n")
        .unwrap()
        .1
        .to_owned();
    let chat_id = msg.chat.id;
    bot.send_message(
        chat_id,
        format!("Братулец, это все, что я поддерживаю\n{}", descr),
    )
    .await?;
    dialogue.reset().await.unwrap();
    Ok(())
}

pub async fn init_handler_global(
    bot: Bot,
    msg: Message,
    dialogue: GlobalDialogue,
    cmd: Commands,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    match cmd {
        Commands::Start => {
            let descr = Commands::descriptions()
                .to_string()
                .split_once("\n\n")
                .unwrap()
                .1
                .to_owned();
            let resp = format!(
                "Братулец, ну что-ж, начнем 👽\nНабор команд,которых я поддреживаю:\n{}",
                descr
            );
            bot.send_message(chat_id, resp).await?;
            dialogue
                .update(Global::AuthState(Auth::Init))
                .await
                .unwrap();
            bot.send_message(chat_id, "Итак, вводи /startauth").await?;
        }
        _ => {
            bot.send_message(chat_id, "Братулец, введи /start").await?;
            dialogue.reset().await.unwrap();
        }
    }

    Ok(())
}
