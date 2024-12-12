use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenInterface, TokenAccount, TransferChecked},
};

use crate::states::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = initializer,
        associated_token::token_program = token_program,
    )]
    pub initializer_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"state".as_ref(), &escrow.seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, deposit_amount: u64) -> Result<()> {
        self.escrow.remaining_amount += deposit_amount;
        transfer_checked(
            self.into_deposit_context(),
            deposit_amount,
            self.mint.decimals,
        )
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.initializer_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
