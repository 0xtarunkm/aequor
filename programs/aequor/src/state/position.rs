use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 4)]
pub struct Position {
    pub aequor: Pubkey,
    pub position_mint: Pubkey,
    pub liquidity: u128,
    pub tick_lower_index: i32,
    pub tick_upper_index: i32,

    pub fee_growth_checkpoint_a: u128,
    pub fee_owed_a: u64,

    pub fee_growth_checkpoint_b: u128,
    pub fee_owed_b: u64,

    pub position_bump: u8,
}
