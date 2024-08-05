use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub seed: u64,
    pub bump: u8,
    pub initializer: Pubkey,
    pub mint_zeus: Pubkey,
    pub one_time_amount: u64,
    pub max_amount: u64,
    pub deposit_amount: u64,
    pub remaining_amount: u64,
}

impl Space for Escrow {
    // First 8 Bytes are Discriminator (u64)
    const INIT_SPACE: usize = 8 + 8 + 1 + 32 + 32 + 8 + 8 + 8 + 8;
}
