use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::states::Escrow;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_zeus: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_zeus,
        associated_token::authority = initializer
    )]
    pub initializer_ata_zeus: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = initializer,
        space = Escrow::INIT_SPACE,
        seeds = [b"state".as_ref(), &seed.to_le_bytes()],
        bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = mint_zeus,
        associated_token::authority = escrow
    )]
    pub vault: Account<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_escrow(
        &mut self,
        seed: u64,
        bumps: &InitializeBumps,
        one_time_amount: u64,
        max_amount: u64,
        deposit_amount: u64,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed,
            bump: bumps.escrow,
            initializer: self.initializer.key(),
            mint_zeus: self.mint_zeus.key(),
            one_time_amount,
            max_amount,
            deposit_amount,
            remaining_amount: deposit_amount,
        });
        Ok(())
    }

    pub fn deposit(&mut self, deposit_amount: u64) -> Result<()> {
        transfer_checked(
            self.into_deposit_context(),
            deposit_amount,
            self.mint_zeus.decimals,
        )
    }

    fn into_deposit_context(&self) -> CpiContext<'_, '_, '_, 'info, TransferChecked<'info>> {
        let cpi_accounts = TransferChecked {
            from: self.initializer_ata_zeus.to_account_info(),
            mint: self.mint_zeus.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.initializer.to_account_info(),
        };
        CpiContext::new(self.token_program.to_account_info(), cpi_accounts)
    }
}
