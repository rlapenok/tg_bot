use std::sync::{Arc, Mutex};

use teloxide::{
    payloads::SendMessageSetters,
    requests::{Requester, ResponseResult},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};

use crate::{
    build_dispatcher::{create_chema::SelecWalletsDialogue, dialogues::dialogues::SelectWallets},
    client::client::Client,
};

pub async fn get_wallet_handler_select_wallets(
    bot: Bot,
    msg: Message,
    dialogue_select_wallet: SelecWalletsDialogue,
    mut client: Client,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;
    //Get all keys
    match client.get_keys().await {
        Ok(mut stream) => {
            let mut count = 0;
            let buttons = Arc::new(Mutex::new(Vec::new()));
            loop {
                match stream.message().await {
                    //Stream open and recive Ok msg
                    Ok(row_message) => {
                        match row_message {
                            Some(msg) => {
                                log::info!("@@@ New msg from Router service @@@");
                                if msg.len != 0 {
                                    count += 1;
                                    let button = InlineKeyboardButton::callback(
                                        format!("Wallet № {}", count),
                                        count.to_string(),
                                    );
                                    buttons.clone().lock().unwrap().push(button)
                                } else {
                                    bot.send_message(chat_id, "Нет приватных ключей, если хочешь, то можешь кидать их сюда").await?;
                                    break;
                                }
                            }
                            //Stream closed
                            None => {
                                log::info!("@@@ Stream from Router service are closed @@@");
                                if buttons.clone().lock().unwrap().is_empty() {
                                    break;
                                } else {
                                    let mut new_buttons = Vec::new();
                                    buttons
                                        .lock()
                                        .unwrap()
                                        .iter()
                                        .for_each(|button| new_buttons.push(vec![button.clone()]));
                                    let button_all_wallets =
                                        vec![InlineKeyboardButton::callback("All Waleets", "0")];
                                        
                                    new_buttons.push(button_all_wallets);
                                    let keyboard = InlineKeyboardMarkup::new(new_buttons);
                                    dialogue_select_wallet
                                        .update(SelectWallets::Wallets(Vec::new()))
                                        .await
                                        .unwrap();
                                    bot.send_message(chat_id, "Select Wallets")
                                        .reply_markup(keyboard)
                                        .await?;

                                    break;
                                }
                            }
                        }
                    }
                    //Stream open and recive Err msg
                    Err(status) => {
                        log::error!("@@@ {} @@@", status.message());
                        let message = "DumbBase service are down";
                        bot.send_message(chat_id, message).await?;
                    }
                }
            }
        }
        //Err on the Router
        Err(status) => {
            log::error!("@@@ {} @@@", status.message());
            let msg = "Router service are down";
            bot.send_message(chat_id, msg).await?;
        }
    }
    Ok(())
}

pub async fn select_wallets_handler_select_wallets(
    bot: Bot,
    call_back: CallbackQuery,
    dialogue_select_wallet:SelecWalletsDialogue,
) -> ResponseResult<()> {
    let chat_id = call_back.message.as_ref().unwrap().chat.id;
    let call_back_wallet = call_back.data.unwrap().parse::<usize>().unwrap();
    //Use for delet previus keyboard
    let msg_id=call_back.message.as_ref().unwrap().id;
    //Previus keyboard
    let keyboard=& mut call_back.message.unwrap().reply_markup().unwrap().inline_keyboard.clone();
    //Matching SelectWalletDialogie
    match dialogue_select_wallet.get().await.unwrap(){
        Some(state)=>{
            match  state {
                SelectWallets::Init=>{}
                SelectWallets::Wallets(mut storage)=>{
                    //Search in storage
                    match storage.binary_search(&call_back_wallet){
                        Ok(_)=>{
                            log::warn!("@@@ Key are selected @@@");
                            bot.send_message(chat_id, "Этот кошелек уже выбран").await?;
                        }
                        //Can't search
                        Err(_)=>{
                            if call_back_wallet!=0{
                                log::info!("@@@ Key are not selected");
                                let wallet=format!("{}",call_back_wallet);
                                //Get select button
                                let mut button=&mut keyboard.get_mut(call_back_wallet-1).unwrap()[0];
                                //Update text in button
                                button.text=format!("{} ✅",&button.text);
                                log::info!("@@@ Select Wallet № {}",wallet);
                                //Delete previus keyboard
                                bot.delete_message(chat_id, msg_id).await?;
                                //Send new keyboard with select wwallet
                                bot.send_message(chat_id, "Select Wallet").reply_markup(InlineKeyboardMarkup::new(keyboard.clone())).await?;
                                //Update storage in SelectWalletDialogue
                                storage.push(call_back_wallet);
                                //Sort storage in SelectWalletDialogue
                                storage.sort();
                                //Update SelectWalletsDialogue with new storage
                                dialogue_select_wallet.update(SelectWallets::Wallets(storage.clone())).await.unwrap();
                            }else{
                                log::info!("@@@ All keys are selected");
                            bot.send_message(chat_id, "Выбраны все кошельки").await?;
                                bot.delete_message(chat_id, msg_id).await?;
                            }

                        }
                    }
                }
                
            }
        }
        None=>{}
    }
    Ok(())
}
