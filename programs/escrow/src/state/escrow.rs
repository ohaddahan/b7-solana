use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone, PartialEq)]
pub enum EscrowStatus {
    Initialized,
    Side1Fulfilled,
    Side2Fulfilled,
    Completed,
}

impl Default for EscrowStatus {
    fn default() -> Self {
        Self::Initialized
    }
}

#[account]
#[derive(Default, Debug)]
pub struct Escrow {
    pub bump: u8,
    pub uuid: String,
    pub side_1: Pubkey,
    pub side_1_mint: Pubkey,
    pub side_1_amount: u64,
    pub side_2: Pubkey,
    pub side_2_mint: Pubkey,
    pub side_2_amount: u64,
    pub status: EscrowStatus,
}

impl Escrow {
    pub const PREFIX: &'static str = "ESCROW";

    pub const SIZE: usize = 8 + /* discriminator */
        std::mem::size_of::<u8>() + /* bump */
        100 + /* uuid */
        std::mem::size_of::<Pubkey>() + /* side_1 */
        std::mem::size_of::<Pubkey>() + /* side_1_mint */
        std::mem::size_of::<u64>() + /* side_1_amount */
        std::mem::size_of::<Pubkey>() + /* side_2 */
        std::mem::size_of::<Pubkey>() + /* side_2_mint */
        std::mem::size_of::<u64>() + /* side_2_amount */
        10; /* padding */
}
