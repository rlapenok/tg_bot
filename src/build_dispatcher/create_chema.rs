use teloxide::{
    dispatching::{dialogue::InMemStorage, DpHandlerDescription, HandlerExt, UpdateFilterExt},
    dptree::case,
    filter_command,
    prelude::{DependencyMap, Dialogue},
    types::{Message, Update},
    RequestError,
};

use super::{
    commands::Commands,
    dialogues::{
        auth_handler::{check_code_handler_auth, other_msg_hanler_auth, init_handler_auth},
        dialogues::{Auth, Global},
        global_handler::{init_handler_global, other_msg_hanler_global, get_wallet_handler_global},
    },
};
pub type AuthDialogue = Dialogue<Auth, InMemStorage<Auth>>;
pub type GlobalDialogue = Dialogue<Global, InMemStorage<Global>>;
pub fn create_schema() -> teloxide::prelude::Handler<
    'static,
    DependencyMap,
    Result<(), RequestError>,
    DpHandlerDescription,
> {
    let auth_dialogue = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<Auth>, Auth>()
        .branch(case![Auth::Init].chain(filter_command::<Commands, _>().endpoint(init_handler_auth)))
        .branch(case![Auth::CheckCode].endpoint(check_code_handler_auth))
        .branch(Update::filter_message().endpoint(other_msg_hanler_auth));

    let global = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<Global>, Global>()
        .branch(
            case![Global::Init]
                .chain(filter_command::<Commands, _>().endpoint(init_handler_global))
                .branch(Update::filter_message().endpoint(other_msg_hanler_global)),
        )
        .branch(case![Global::AuthState(auth)].chain(auth_dialogue))
        .branch(case![Global::GetWallet].endpoint(get_wallet_handler_global))
        .branch(Update::filter_message().endpoint(other_msg_hanler_global));
    global
}
