#[derive(Clone, Default)]
pub enum Auth {
    #[default]
    Init,
    CheckCode,
    End,
}

#[derive(Clone, Default)]
pub enum Global {
    #[default]
    Init,
    DeadLock,
    AuthState(Auth),
    GetWallet,
}
