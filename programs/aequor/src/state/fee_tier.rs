use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 3)]
pub struct FeeTier {
    pub aequors_config: Pubkey,
    pub tick_spacing: u16,
    pub default_fee_rate: u16,
}
