use anchor_lang::prelude::*;
mod contexts;
use contexts::*;
mod states;
mod error;
declare_id!("2fTfygBAg6pcHTffQ916y3GEZybMTa2KRyu9ThXuuRzo");
#[program]
pub mod anchor_escrow {

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        one_time_amount: u64,
        max_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(seed, &ctx.bumps, one_time_amount, max_amount, deposit_amount)?;
        ctx.accounts.deposit(deposit_amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()
    }

    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()
    }
}
