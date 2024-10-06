use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken, token::{self, Mint, Token, TokenAccount}
};
use drift::{
    cpi::{
        accounts::Withdraw,
        withdraw
    },
    program::Drift,
    state::{
        state::State as DriftState, 
        user::{
            User as DriftUser, 
            UserStats as DriftUserStats
        }
    }
};
use crate::{
    constants::{DRIFT_MARKET_INDEX_USDC, USDC_MINT_ADDRESS}, 
    errors::ErrorCode, 
    state::Vault
};

#[derive(Accounts)]
pub struct WithdrawUsdc<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner
    )]
    pub vault: Box<Account<'info, Vault>>,

    #[account(
        init,
        seeds = [vault.key().as_ref(), usdc_mint.key().as_ref()],
        bump,
        payer = owner,
        token::mint = usdc_mint,
        token::authority = vault
    )]
    pub vault_usdc: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = usdc_mint,
        associated_token::authority = owner
    )]
    pub owner_usdc: Box<Account<'info, TokenAccount>>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    #[account(
        mut,
        seeds = [b"drift_state"],
        seeds::program = drift_program.key(),
        bump
    )]
    pub drift_state: Box<Account<'info, DriftState>>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    #[account(
        mut,
        seeds = [b"user", vault.key().as_ref(), (0u16).to_le_bytes().as_ref()],
        seeds::program = drift_program.key(),
        bump
    )]
    pub drift_user: AccountLoader<'info, DriftUser>,
    
    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    #[account(
        mut,
        seeds = [b"user_stats", vault.key().as_ref()],
        seeds::program = drift_program.key(),
        bump
    )]
    pub drift_user_stats: AccountLoader<'info, DriftUserStats>,
    
    #[account(
        mut,
        seeds = [b"spot_market_vault", (DRIFT_MARKET_INDEX_USDC).to_le_bytes().as_ref()],
        seeds::program = drift_program.key(),
        token::mint = usdc_mint,
        bump,
    )]
    pub spot_market_vault: Box<Account<'info, TokenAccount>>,
    
    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    pub drift_signer: UncheckedAccount<'info>,

    #[account(
        constraint = usdc_mint.key() == USDC_MINT_ADDRESS @ ErrorCode::InvalidMintAddress
    )]
    pub usdc_mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, Token>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub drift_program: Program<'info, Drift>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    pub const_account: UncheckedAccount<'info>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    pub additional_account: UncheckedAccount<'info>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    pub spot_market_sol: UncheckedAccount<'info>,

    /// CHECK: This account is passed through to the Drift CPI, which performs the security checks
    #[account(mut)]
    pub spot_market_usdc: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn withdraw_usdc_handler(
    ctx: Context<WithdrawUsdc>, 
    amount_cents: u64
) -> Result<()> {
    let vault_bump = ctx.accounts.vault.bump;
    let owner = ctx.accounts.owner.key();
    let seeds = &[
        b"vault",
        owner.as_ref(),
        &[vault_bump]
    ];
    let signer_seeds = &[&seeds[..]];

    // Build Drift Withdraw CPI
    let mut cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.drift_program.to_account_info(),
        Withdraw {
            state: ctx.accounts.drift_state.to_account_info(),
            user: ctx.accounts.drift_user.to_account_info(),
            user_stats: ctx.accounts.drift_user_stats.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
            spot_market_vault: ctx.accounts.spot_market_vault.to_account_info(),
            drift_signer: ctx.accounts.drift_signer.to_account_info(),
            user_token_account: ctx.accounts.vault_usdc.to_account_info(),
            token_program: ctx.accounts.token_program.to_account_info(),
        },
        signer_seeds
    );

    // Add remaining accounts and send CPI
    cpi_ctx.remaining_accounts = vec![
        ctx.accounts.const_account.to_account_info(),
        ctx.accounts.additional_account.to_account_info(),
        ctx.accounts.spot_market_sol.to_account_info(),
        ctx.accounts.spot_market_usdc.to_account_info(),
    ];

    withdraw(cpi_ctx, DRIFT_MARKET_INDEX_USDC, amount_cents, true)?;

    // Transfer USDC to owner's ATA

    token::transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(), 
            token::Transfer { 
                from: ctx.accounts.vault_usdc.to_account_info(), 
                to: ctx.accounts.owner_usdc.to_account_info(), 
                authority: ctx.accounts.vault.to_account_info()
            }, 
            signer_seeds
        ),
        amount_cents
    )?;

    // Close vault USDC

    let cpi_ctx_close = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        token::CloseAccount {
            account: ctx.accounts.vault_usdc.to_account_info(),
            destination: ctx.accounts.owner_usdc.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        },
        signer_seeds
    );
    token::close_account(cpi_ctx_close)?;

    Ok(())
}