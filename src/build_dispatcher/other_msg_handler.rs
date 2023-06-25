use teloxide::{
    requests::{Requester, ResponseResult},
    types::Message,
    Bot,
};

pub async fn other_msg_handler(bot: Bot, msg: Message) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    bot.send_message(chat_id, "Братулец, что-то ты ввел не так => го в Вагнер 🇷🇺")
        .await?;

    Ok(())
}
