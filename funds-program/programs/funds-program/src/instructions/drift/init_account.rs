use anchor_lang::prelude::*;
use drift_sdk::cpi::{InitializeUser, initialize_user, InitializeUserStats, initialize_user_stats};

#[derive(Accounts)]
pub struct DriftUserInit<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump,
    )]
    pda_account: SystemAccount<'info>,

    #[account(mut)]
    pub user: AccountInfo<'info>,
    #[account(mut)]
    pub user_stats: AccountInfo<'info>,
    #[account(mut)]
    pub state: AccountInfo<'info>,
    #[account(mut, signer)]
    pub authority: AccountInfo<'info>,
    #[account(mut, signer)]
    pub payer: AccountInfo<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,

    #[account(mut)]
    owner: SystemAccount<'info>,

    /// CHECK: This is the Drift program ID
    pub drift_program: AccountInfo<'info>,
}

pub fn initialize_drift_user_handler(ctx: Context<DriftUserInit>) -> Result<()> {
    let drift_program_id = ctx.accounts.drift_program.key();
    
    let seed = ctx.accounts.owner.key();
    let bump_seed = ctx.bumps.pda_account;
    let signer_seeds: &[&[&[u8]]] = &[&[b"vault", seed.as_ref(), &[bump_seed]]];

    let create_user_stats_cpi_context = CpiContext::new_with_signer(
        drift_program_id,
        InitializeUserStats {
            user_stats: ctx.accounts.user_stats.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
    );
    
    initialize_user_stats(create_user_stats_cpi_context)?;

    let create_user_cpi_context = CpiContext::new_with_signer(
        drift_program_id,
        InitializeUser {
            user: ctx.accounts.user.to_account_info(),
            user_stats: ctx.accounts.user_stats.to_account_info(),
            state: ctx.accounts.state.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
            payer: ctx.accounts.payer.to_account_info(),
            rent: ctx.accounts.rent.to_account_info(),
            system_program: ctx.accounts.system_program.to_account_info(),
        },
        signer_seeds,
    );
 
    initialize_user(create_user_cpi_context)?;
    Ok(())
}