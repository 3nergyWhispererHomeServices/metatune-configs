use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid authority")]
    InvalidAuthority,
    #[msg("Missing remaining accounts for config entry")]
    MissingRemainingAccountsForConfigEntry,
    #[msg("Invalid stake pool account")]
    InvalidStakePoolAccount,
    #[msg("Invalid reward center pool account")]
    InvalidRewardCenterPoolAccount,
    #[msg("Invalid pool authority")]
    InvalidPoolAuthority,
    #[msg("Pool address not found in config")]
    PoolAddressNotFound,
    #[msg("Invalid pool address in config")]
    InvalidConfigPoolAddress,

    // Misc
    #[msg("Cardinal Protocols are shutting down. Please read latest twitter post for more information")]
    ProtocolsShutdown,
}
