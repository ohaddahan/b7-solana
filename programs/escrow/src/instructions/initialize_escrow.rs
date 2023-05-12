use crate::state::escrow::{Escrow, EscrowStatus};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::Token;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct InitializeEscrowArgs {
    pub uuid: String,
    pub side_1_mint: Pubkey,
    pub side_1_amount: u64,
    pub side_2_mint: Pubkey,
    pub side_2_amount: u64,
}

#[derive(Accounts)]
#[instruction(args: InitializeEscrowArgs)]
pub struct InitializeEscrowContext<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
    init,
    payer = signer,
    seeds=[Escrow::PREFIX.as_bytes(), args.uuid.as_bytes()],
    space=Escrow::SIZE,
    bump
    )]
    pub escrow: Account<'info, Escrow>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_escrow(
    ctx: Context<InitializeEscrowContext>,
    args: InitializeEscrowArgs,
) -> Result<()> {
    let escrow = &mut ctx.accounts.escrow;

    escrow.bump = ctx.bumps.get("escrow").unwrap().clone();
    escrow.uuid = args.uuid;
    escrow.side_1_mint = args.side_1_mint;
    escrow.side_1_amount = args.side_1_amount;
    escrow.side_2_mint = args.side_2_mint;
    escrow.side_2_amount = args.side_2_amount;
    escrow.status = EscrowStatus::Initialized;

    Ok(())
}
