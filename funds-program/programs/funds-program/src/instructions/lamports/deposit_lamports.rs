use anchor_lang::prelude::*;
use anchor_lang::system_program;
use anchor_spl::{
    token,
    token::{Mint, Token, SyncNative}, 
    token::TokenAccount
};
use drift_sdk::{
    accounts::{
        State as DriftState, 
        User as DriftUser,
        UserStats as DriftUserStats
    }, 
    cpi::deposit,
    Deposit
};
use crate::{
    constants::{DRIFT_PROGRAM_ID, WSOL_MINT_ADDRESS},
    errors::ErrorCode,
    state::Vault
};

#[derive(Accounts)]
pub struct DepositLamports<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        init,
        seeds = [vault.key().as_ref(), wsol_mint.key().as_ref()],
        bump,
        payer = owner,
        token::mint = wsol_mint,
        token::authority = vault
    )]
    pub vault_wsol: Account<'info, TokenAccount>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [b"drift_state"],
        seeds::program = drift_program.key(),
        bump
    )]
    pub state: Box<Account<'info, DriftState>>,
    
    #[account(
        mut,
        seeds = [b"user", vault.key().as_ref(), (0u16).to_le_bytes().as_ref()],
        seeds::program = drift_program.key(),
        bump
    )]
    pub user: AccountLoader<'info, DriftUser>,

    #[account(
        mut,
        seeds = [b"user_stats", vault.key().as_ref()],
        seeds::program = drift_program.key(),
        bump
    )]
    pub user_stats: Account<'info, DriftUserStats>,

    #[account(
        mut,
        seeds = [b"spot_market_vault", (1u16).to_le_bytes().as_ref()], // 1 for SOL
        seeds::program = drift_program.key(),
        bump,
    )]
    pub spot_market_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = wsol_mint.key() == WSOL_MINT_ADDRESS @ ErrorCode::InvalidMintAddress
    )]
    pub wsol_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,

    /// CHECK: Account is safe once the address is correct
    #[account(
        constraint = drift_program.key() == DRIFT_PROGRAM_ID @ ErrorCode::InvalidDriftProgram
    )]
    pub drift_program: UncheckedAccount<'info>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    pub const_account: UncheckedAccount<'info>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    #[account(mut)]
    pub spot_market: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>
}

pub fn deposit_lamports_handler(
    ctx: Context<DepositLamports>, 
    amount: u64
) -> Result<()> {
    let vault_bump = ctx.accounts.vault.bump;
    let owner = ctx.accounts.owner.key();
    let seeds = &[
        b"vault",
        owner.as_ref(),
        &[vault_bump]
    ];
    let signer_seeds = &[&seeds[..]];

    msg!("deposit_lamprots: Wrapping {} lamports", amount);

    // Transfer SOL from user to vault wSOL account
    let cpi_ctx_transfer = CpiContext::new(
        ctx.accounts.system_program.to_account_info(),
        system_program::Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault_wsol.to_account_info(),
        },
    );
    system_program::transfer(cpi_ctx_transfer, amount)?;

    // Sync the native token to reflect the new SOL balance as wSOL
    let cpi_ctx_sync = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SyncNative {
            account: ctx.accounts.vault_wsol.to_account_info(),
        },
    );
    token::sync_native(cpi_ctx_sync)?;

    msg!("deposit_lamprots: Deposit {} wSol lamports into Drift", amount);

    // Build Drift Deposit CPI
    let mut cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.drift_program.to_account_info(),
        Deposit {
            state: ctx.accounts.state.to_account_info(),
            user: ctx.accounts.user.to_account_info(),
            user_stats: ctx.accounts.user_stats.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
            spot_market_vault: ctx.accounts.spot_market_vault.to_account_info(),
            user_token_account: ctx.accounts.vault_wsol.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        signer_seeds
    );

    // Add additional_account and market_vault as remaining accounts
    cpi_ctx.remaining_accounts = vec![
        ctx.accounts.const_account.to_account_info(),
        ctx.accounts.spot_market.to_account_info(),
    ];

    deposit(cpi_ctx, 1, amount, false)?;

    msg!("deposit_lamprots: Closing wSol vault");

    let cpi_ctx_close = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account: ctx.accounts.vault_wsol.to_account_info(),
            destination: ctx.accounts.owner.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds
    );
    token::close_account(cpi_ctx_close)?;

    msg!("deposit_lamprots: Done");

    Ok(())
}