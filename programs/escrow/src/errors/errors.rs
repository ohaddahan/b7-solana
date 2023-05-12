use anchor_lang::prelude::*;

#[error_code]
pub enum Errors {
    #[msg("Mint Mismatch")]
    MintMismatch,
    #[msg("escrow_side_2_token_account.mint != side_1_token_account.mint")]
    MintMismatch1,
    #[msg("escrow_side_1_token_account.mint != side_2_token_account.mint")]
    MintMismatch2,
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Escrow Status Not Side2Fulfilled")]
    EscrowStatusNotSide2Fulfilled,
}
