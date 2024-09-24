use anchor_lang::prelude::*;
use crate::state::Vault;

#[derive(Accounts)]
pub struct StakeLamports<'info> {
    #[account(
        mut,
        seeds = [b"vault", owner.key().as_ref()],
        bump = vault.bump,
        has_one = owner,
    )]
    pub vault: Account<'info, Vault>,

    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>
}

pub fn stake_lamports_handler(
    _ctx: Context<StakeLamports>, 
    amount: u64
) -> Result<()> {
    msg!("Staking {} lamports", amount);

    // TODO - Implement
    

    msg!("Lamports staked");

    Ok(())
}