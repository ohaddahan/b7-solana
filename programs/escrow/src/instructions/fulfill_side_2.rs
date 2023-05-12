use crate::state::escrow::{Escrow, EscrowStatus};
use crate::utils::helpers::transfer_token;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct FullFillSide2Context<'info> {
    #[account(mut)]
    pub side_2: Signer<'info>,
    #[account(
    mut,
    associated_token::mint=side_2_mint,
    associated_token::authority=side_2
    )]
    pub side_2_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    seeds=[Escrow::PREFIX.as_bytes(), escrow.uuid.as_bytes()],
    has_one=side_2_mint,
    bump=escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
    init_if_needed,
    payer = side_2,
    associated_token::mint=side_2_mint,
    associated_token::authority=escrow
    )]
    pub escrow_token_account: Box<Account<'info, TokenAccount>>,
    pub side_2_mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn fulfill_side_2(ctx: Context<FullFillSide2Context>) -> Result<()> {
    let side_2 = &mut ctx.accounts.side_2;
    let token_program = &mut ctx.accounts.token_program;
    let escrow = &mut ctx.accounts.escrow;
    let side_2_token_account = &mut ctx.accounts.side_2_token_account;
    let escrow_token_account = &mut ctx.accounts.escrow_token_account;

    escrow.side_2 = *side_2.to_account_info().key;
    escrow.status = EscrowStatus::Side2Fulfilled;

    transfer_token(
        side_2_token_account.to_account_info(),
        escrow_token_account.to_account_info(),
        token_program.to_account_info(),
        side_2.to_account_info(),
        escrow.side_2_amount,
    )?;
    Ok(())
}
