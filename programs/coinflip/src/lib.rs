use anchor_lang::prelude::*;
use anchor_lang::solana_program::{clock};
use std::mem::size_of;
use solana_program::{program::invoke, program::invoke_signed, system_instruction};

declare_id!("B6cP8oy6k2axMwCwgifeAq5BGnzB77nvvTddvtKND5uQ");

#[program]
pub mod coinflip {
    use super::*;

    pub const VAULT_SEED: &[u8] = b"VAULT_SEED";
    pub const USER_STATE_SEED: &[u8] = b"USER_STATE_SEED";

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("initialize");
        let accts = ctx.accounts;
        accts.user_state.is_initialized = 1;
        accts.user_state.vault = accts.vault.key();

        Ok(())
    }

    pub fn coinflip(ctx: Context<CoinFlip>, amount: u64, rand: u32) -> Result<u64> {
        let c = clock::Clock::get().unwrap();
        let cc = c.unix_timestamp + rand as i64;
        let r = (cc % 2) as u8;

        let bump = ctx.bumps.get("vault").unwrap();
        let accts = ctx.accounts;

        if r == 0  { // win case
            accts.user_state.last_coinflip_res = 0;
            let rewards = amount * 2;
            accts.user_state.last_rewards = rewards;

            // send to user
            invoke_signed(
                &system_instruction::transfer(&accts.vault.key(), &accts.user.key(), rewards),
                &[
                    accts.vault.to_account_info().clone(),
                    accts.user.clone(),
                    accts.system_program.to_account_info().clone(),
                ],
                &[&[VAULT_SEED, &[*bump]]],
            )?;
        } else { // lose case
            accts.user_state.last_coinflip_res = 1;
            accts.user_state.last_rewards = amount;

            // send to treasury
            invoke(
                &system_instruction::transfer(&accts.user.key(), &accts.vault.key(), amount),
                &[
                    accts.user.to_account_info().clone(),
                    accts.vault.clone(),
                    accts.system_program.to_account_info().clone(),
                ],
            )?;

        }

        Ok(amount)
    }
}


#[account]
#[derive(Default)]
pub struct UserState {
    // to avoid reinitialization attack
    pub is_initialized: u8,
    pub vault: Pubkey,
    pub last_coinflip_res: u8, // 0 : win, 1 : loose, 2 : draw
    pub last_rewards: u64 // 0 : win, 1 : loose, 2 : draw
}


#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_STATE_SEED],
        bump,
        space = 8 + size_of::<UserState>(),
        payer = authority,
    )]
    pub user_state: Account<'info, UserState>,

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be checked with vault address
    pub vault: AccountInfo<'info>,

    pub system_program: Program<'info, System>,

}

#[derive(Accounts)]
pub struct CoinFlip<'info> {

    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump
    )]
    /// CHECK: this should be checked with vault address
    pub vault: AccountInfo<'info>,

    #[account(mut)]
    pub user_state: Account<'info, UserState>,

    #[account(mut)]
    /// CHECK: this should be checked with valid address
    pub user: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum SpinError {
    #[msg("Count Overflow To Add Item")]
    CountOverflowAddItem,

    #[msg("Index Overflow To Set Item")]
    IndexOverflowSetItem,
}