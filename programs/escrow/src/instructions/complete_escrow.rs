use crate::errors::errors::Errors;
use crate::state::escrow::{Escrow, EscrowStatus};
use crate::utils::helpers::transfer_token_pda;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
pub struct CompleteEscrowContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: Checked by escrow
    pub side_1: AccountInfo<'info>,
    #[account(
    init_if_needed,
    payer = signer,
    associated_token::mint=side_2_mint,
    associated_token::authority=side_1
    )]
    pub side_1_token_account: Box<Account<'info, TokenAccount>>,
    /// CHECK: Checked by escrow
    pub side_2: AccountInfo<'info>,
    #[account(
    init_if_needed,
    payer = signer,
    associated_token::mint=side_1_mint,
    associated_token::authority=side_2
    )]
    pub side_2_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    seeds=[Escrow::PREFIX.as_bytes(), escrow.uuid.as_bytes()],
    has_one=side_1,
    has_one=side_2,
    has_one=side_1_mint,
    has_one=side_2_mint,
    constraint=escrow.status==EscrowStatus::Side2Fulfilled @ Errors::EscrowStatusNotSide2Fulfilled,
    bump=escrow.bump
    )]
    pub escrow: Account<'info, Escrow>,
    #[account(
    mut,
    associated_token::mint=escrow.side_1_mint,
    associated_token::authority=escrow
    )]
    pub escrow_side_1_token_account: Box<Account<'info, TokenAccount>>,
    #[account(
    mut,
    associated_token::mint=escrow.side_2_mint,
    associated_token::authority=escrow
    )]
    pub escrow_side_2_token_account: Box<Account<'info, TokenAccount>>,
    pub side_1_mint: Box<Account<'info, Mint>>,
    pub side_2_mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn complete_escrow(ctx: Context<CompleteEscrowContext>) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;
    let token_program = &mut ctx.accounts.token_program;
    let escrow_side_1_token_account = &mut ctx.accounts.escrow_side_1_token_account;
    let escrow_side_2_token_account = &mut ctx.accounts.escrow_side_2_token_account;
    let side_1_token_account = &mut ctx.accounts.side_1_token_account;
    let side_2_token_account = &mut ctx.accounts.side_2_token_account;

    escrow.status = EscrowStatus::Completed;

    let seeds = &[
        Escrow::PREFIX.as_bytes(),
        escrow.uuid.as_bytes(),
        &[escrow.bump],
    ];

    require_keys_eq!(
        escrow_side_2_token_account.mint,
        side_1_token_account.mint,
        Errors::MintMismatch1
    );
    require_keys_eq!(
        escrow_side_1_token_account.mint,
        side_2_token_account.mint,
        Errors::MintMismatch2
    );
    transfer_token_pda(
        escrow_side_2_token_account.to_account_info(),
        side_1_token_account.to_account_info(),
        token_program.to_account_info(),
        escrow.to_account_info(),
        escrow.side_2_amount,
        &[seeds],
    )?;
    Ok(())
}
