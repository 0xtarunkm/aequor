use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 2)]
pub struct Aequor {
    pub aequors_config: Pubkey,
    pub tick_spacing: u16,
    pub fee_tier_index_seed: [u8; 2],

    pub fee_rate: u16,
    pub protocol_fee_rate: u16,

    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,

    pub protocol_fee_a: u64,
    pub protocol_fee_b: u64,

    pub mint_a: Pubkey,
    pub vault_a: Pubkey,
    pub fee_growth_global_a: u128,

    pub mint_b: Pubkey,
    pub vault_b: Pubkey,
    pub fee_growth_global_b: u128,

    pub mint_lp: Pubkey,

    pub bump: u8,
}
