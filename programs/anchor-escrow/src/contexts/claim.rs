use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        transfer_checked, Mint, Token, TokenAccount, TransferChecked,
    },
};
use crate::error::ErrorCode;
use crate::states::{Escrow, Zeusfrens};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    claimer: Signer<'info>,
    mint_zeus: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = mint_zeus,
        associated_token::authority = claimer
    )]
    claimer_ata_zeus: Account<'info, TokenAccount>,
    #[account(
        mut,
        has_one = mint_zeus,
        seeds=[b"state", escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = claimer,
        seeds = [b"zeusfrens", claimer.key().as_ref(), escrow.key().as_ref()],
        space = Zeusfrens::INIT_SPACE,
        bump,
    )]
    pub zeusfrens: Account<'info, Zeusfrens>,
    #[account(
        mut,
        associated_token::mint = mint_zeus,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"state",
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        self.update_zeusfrens()?;

        transfer_checked(
            self.into_claim_context().with_signer(&signer_seeds),
            self.escrow.one_time_amount,
            self.mint_zeus.decimals,
        )
    }

    fn into_claim_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_zeus.to_account_info(),
            to: self.claimer_ata_zeus.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn update_zeusfrens(&mut self) -> Result<()> {
        if self.zeusfrens.claimed_amount + self.escrow.one_time_amount > self.escrow.max_amount {
            return Err(ErrorCode::OutOfMaxAmount.into());
        }
        if self.escrow.remaining_amount < self.escrow.one_time_amount {
            return Err(ErrorCode::NoRemainingAmount.into());
        }
        self.escrow.remaining_amount -= self.escrow.one_time_amount;
        self.zeusfrens.set_inner(Zeusfrens {
            claimed_amount: self.zeusfrens.claimed_amount + self.escrow.one_time_amount,
        });
        Ok(())
    }
}
