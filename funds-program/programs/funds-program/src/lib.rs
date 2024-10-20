use anchor_lang::prelude::*;

mod state;
mod errors;
mod constants;
mod instructions;
use instructions::*;

declare_id!("6JjHXLheGSNvvexgzMthEcgjkcirDrGduc3HAKB2P1v2");

#[cfg(not(feature = "no-entrypoint"))]
use solana_security_txt::security_txt;
#[cfg(not(feature = "no-entrypoint"))]
security_txt! {
    name: "Quartz",
    project_url: "https://quartzpay.io/",
    contacts: "email:iarla@quartzpay.io",
    policy: "https://github.com/quartz-labs/quartz-app/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/quartz-labs/quartz-app"
}

#[program]
pub mod funds_program {
    use super::*;

    // Config

    pub fn init_user(ctx: Context<InitializeUser>) -> Result<()> {
        init_user_handler(ctx)
    }

    pub fn close_user(ctx: Context<CloseUser>) -> Result<()> {
        close_user_handler(ctx)
    }

    pub fn init_drift_account(ctx: Context<InitDriftAccount>) -> Result<()> {
        init_drift_account_handler(ctx)
    }

    pub fn close_drift_account(ctx: Context<CloseDriftAccount>) -> Result<()> {
        close_drift_account_handler(ctx)
    }

    // Balance

    pub fn withdraw_lamports(ctx: Context<WithdrawLamports>, amount: u64, reduce_only: bool) -> Result<()> {
        withdraw_lamports_handler(ctx, amount, reduce_only)
    }

    pub fn deposit_lamports(ctx: Context<DepositLamports>, amount: u64, reduce_only: bool) -> Result<()> {
        deposit_lamports_handler(ctx, amount, reduce_only)
    }

    pub fn deposit_usdc(ctx: Context<DepositUsdc>, amount_micro_cents: u64, reduce_only: bool) -> Result<()> {
        deposit_usdc_handler(ctx, amount_micro_cents, reduce_only)
    }

    pub fn withdraw_usdc(ctx: Context<WithdrawUsdc>, amount_micro_cents: u64, reduce_only: bool) -> Result<()> {
        withdraw_usdc_handler(ctx, amount_micro_cents, reduce_only)
    }
}
