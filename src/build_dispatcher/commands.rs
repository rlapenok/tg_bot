use teloxide::macros::BotCommands;

#[derive(Clone, BotCommands)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Commands {
    #[command(description = "Если что, то стучись @lapenokr")]
    Help,
    #[command(description = "Начло работы с ботом")]
    Start,
    #[command(description = "Проведение аунетификации с помощью Google Authenticator")]
    StartAuth,
    #[command(description = "Выбор существующих кошельков")]
    GetWallets,
}
