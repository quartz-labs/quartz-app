use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid Quartz account")]
    InvalidQuartzAccount,
    #[msg("Invalid init_payer")]
    InvalidInitPayer,
    #[msg("Insufficent funds for transaction")]
    InsufficientFunds,
    #[msg("Invalid SPL token mint address")]
    InvalidMintAddress,
    #[msg("Invalid Stake Program address")]
    InvalidStakeProgram,
    #[msg("stake_account should be of the Stake Account format")]
    InvalidStakeAccountData,
    #[msg("stake_account should already be initialized")]
    StakeAccountNotInitialized,
    #[msg("stake_account authority should be set to the Vault PDA")]
    InvalidStakeAccountAuthority,
}