#[derive(Clone, Default)]
pub enum Auth {
    #[default]
    Init,
    CheckCode,
    End,
}
#[derive(Clone, Default)]
pub enum UpdateKeys {
    #[default]
    Init,
}
#[derive(Clone, Default)]
pub enum SelectWallets {
    #[default]
    Init,
    Wallets(Vec<usize>),
}

#[derive(Clone, Default)]
pub enum Global {
    #[default]
    Init,
    DeadLock,
    AuthState(Auth),
    SelectWalletState(SelectWallets),
}
