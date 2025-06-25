use anchor_lang::prelude::*;

#[derive(InitSpace)]
#[account(discriminator = 1)]
pub struct AequorConfig {
    pub fee_authority: Pubkey,
    pub collect_fee_authority: Pubkey,
    pub reward_emissions_authority: Pubkey,

    pub protocol_fee_rate: u16,
}
