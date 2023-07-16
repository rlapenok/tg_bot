use teloxide::{
    dispatching::{
        dialogue::{self, InMemStorage},
        DpHandlerDescription, HandlerExt, UpdateFilterExt,
    },
    dptree::case,
    filter_command,
    prelude::{DependencyMap, Dialogue},
    types::{Message, Update},
    RequestError,
};

use super::{
    commands::Commands,
    dialogues::{
        auth_handler::{check_code_handler_auth, init_handler_auth, other_msg_hanler_auth},
        dialogues::{Auth, Global, SelectWallets},
        global_handler::{init_handler_global, other_msg_hanler_global},
        select_wallets_handler::{
            get_wallet_handler_select_wallets, select_wallets_handler_select_wallets,
        },
    },
};
pub type AuthDialogue = Dialogue<Auth, InMemStorage<Auth>>;
pub type GlobalDialogue = Dialogue<Global, InMemStorage<Global>>;
pub type SelecWalletsDialogue = Dialogue<SelectWallets, InMemStorage<SelectWallets>>;

pub fn create_schema() -> teloxide::prelude::Handler<
    'static,
    DependencyMap,
    Result<(), RequestError>,
    DpHandlerDescription,
> {
    let auth_dialogue = Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<Auth>, Auth>()
        .branch(
            case![Auth::Init].chain(filter_command::<Commands, _>().endpoint(init_handler_auth)),
        )
        .branch(case![Auth::CheckCode].endpoint(check_code_handler_auth))
        .branch(Update::filter_message().endpoint(other_msg_hanler_auth));

    let select_wallets_dialogue =
        dialogue::enter::<Update, InMemStorage<SelectWallets>, SelectWallets, _>()
            .branch(
                Update::filter_message().chain(case![SelectWallets::Init].chain(
                    filter_command::<Commands, _>().endpoint(get_wallet_handler_select_wallets),
                )),
            )
            .branch(
                Update::filter_callback_query().chain(
                        
                    case![SelectWallets::Wallets(storage)]
                        .endpoint(select_wallets_handler_select_wallets),
                ),
            );

    let global = dialogue::enter::<Update, InMemStorage<Global>, Global, _>()
        .branch(
            Update::filter_message().chain(
                case![Global::Init]
                    .branch(filter_command::<Commands, _>().endpoint(init_handler_global))
                    .branch(Update::filter_message().endpoint(other_msg_hanler_global)),
            ),
        )
        .branch(case![Global::AuthState(auth)].chain(auth_dialogue))
        .branch(case![Global::SelectWalletState(sw)].chain(select_wallets_dialogue))
        .branch(Update::filter_message().endpoint(other_msg_hanler_global));
    global
}
