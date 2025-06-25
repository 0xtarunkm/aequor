use anchor_lang::prelude::*;

#[event]
pub struct PoolInitialized {
    pub aequor: Pubkey,
    pub aequors_config: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub tick_spacing: u16,
    pub mint_a_ata: Pubkey,
    pub mint_b_ata: Pubkey,
    pub decimals_a: u8,
    pub decimals_b: u8,
    pub initial_sqrt_price: u128,
}