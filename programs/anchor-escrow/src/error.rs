use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq, Eq)]
pub enum ErrorCode {
    #[msg("out of max amount")]
    OutOfMaxAmount,
    #[msg("no remaining amount")]
    NoRemainingAmount,
}
