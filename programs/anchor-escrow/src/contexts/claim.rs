use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        transfer_checked, Mint, TokenInterface, TokenAccount, TransferChecked,
    },
};
use crate::error::ErrorCode;
use crate::states::{Escrow, Frens};

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    claimer: Signer<'info>,
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        init_if_needed,
        payer = claimer,
        associated_token::mint = mint,
        associated_token::authority = claimer,
        associated_token::token_program = token_program,
    )]
    claimer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        has_one = mint,
        seeds=[b"state", escrow.seed.to_le_bytes().as_ref()],
        bump,
    )]
    escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = claimer,
        seeds = [b"frens", claimer.key().as_ref(), escrow.key().as_ref()],
        space = Frens::INIT_SPACE,
        bump,
    )]
    pub frens: Account<'info, Frens>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    associated_token_program: Program<'info, AssociatedToken>,
    token_program: Interface<'info, TokenInterface>,
    system_program: Program<'info, System>,
}

impl<'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {
        let signer_seeds: [&[&[u8]]; 1] = [&[
            b"state",
            &self.escrow.seed.to_le_bytes()[..],
            &[self.escrow.bump],
        ]];

        self.update_frens()?;

        transfer_checked(
            self.into_claim_context().with_signer(&signer_seeds),
            self.escrow.one_time_amount,
            self.mint.decimals,
        )
    }

    fn into_claim_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.claimer_ata.to_account_info(),
            authority: self.escrow.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }

    fn update_frens(&mut self) -> Result<()> {
        if self.frens.claimed_amount + self.escrow.one_time_amount > self.escrow.max_amount {
            return Err(ErrorCode::OutOfMaxAmount.into());
        }
        self.frens.set_inner(Frens {
            claimed_amount: self.frens.claimed_amount + self.escrow.one_time_amount,
        });
        self.escrow.remaining_amount -= self.escrow.one_time_amount;

        Ok(())
    }
}
