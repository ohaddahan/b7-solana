use anchor_lang::prelude::*;
mod errors;
mod instructions;
mod state;
mod utils;

use instructions::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize_escrow(
        ctx: Context<InitializeEscrowContext>,
        args: InitializeEscrowArgs,
    ) -> Result<()> {
        initialize_escrow::initialize_escrow(ctx, args)
    }

    pub fn fulfill_side_1(ctx: Context<FullFillSide1Context>) -> Result<()> {
        fulfill_side_1::fulfill_side_1(ctx)
    }

    pub fn fulfill_side_2(ctx: Context<FullFillSide2Context>) -> Result<()> {
        fulfill_side_2::fulfill_side_2(ctx)
    }

    pub fn complete_escrow(ctx: Context<CompleteEscrowContext>) -> Result<()> {
        complete_escrow::complete_escrow(ctx)
    }
}
