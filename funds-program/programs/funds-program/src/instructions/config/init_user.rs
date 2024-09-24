use anchor_lang::prelude::*;
use anchor_lang::solana_program::stake::program::ID as STAKE_PROGRAM;
use anchor_spl::token::{
    Token,
    TokenAccount,
    Mint
};
use crate::{
    state::Vault,
    constants::USDC_MINT_ADDRESS,
    errors::ErrorCode
};

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(
        init,
        seeds = [b"vault", owner.key().as_ref()],
        bump,
        payer = owner,
        space = Vault::INIT_SPACE
    )]
    pub vault: Account<'info, Vault>,

    #[account(
        init,
        seeds = [b"vault", owner.key().as_ref(), usdc_mint.key().as_ref()],
        bump,
        payer = owner,
        token::mint = usdc_mint,
        token::authority = vault
    )]
    pub vault_usdc: Account<'info, TokenAccount>,

    /// CHECK: TODO - This is actually unsafe, and needs fixing
    pub stake_account: UncheckedAccount<'info>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        constraint = usdc_mint.key() == USDC_MINT_ADDRESS @ ErrorCode::InvalidMintAddress
    )]
    pub usdc_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,

    /// CHECK: This is not dangerous once the account is the stake program
    #[account(
        constraint = stake_program.key() == STAKE_PROGRAM @ ErrorCode::InvalidStakeProgram
    )]
    pub stake_program: UncheckedAccount<'info>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn init_user_handler(ctx: Context<InitializeUser>) -> Result<()> {
    msg!("Initializing account");

    // Set up vault state
    ctx.accounts.vault.owner = ctx.accounts.owner.key();
    ctx.accounts.vault.stake_account = ctx.accounts.stake_account.key();
    ctx.accounts.vault.bump = ctx.bumps.vault;

    // TODO - Check that provided stake_account is initialized, and has vaultPda as the authorities

    Ok(())
}
