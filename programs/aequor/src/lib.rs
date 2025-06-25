use anchor_lang::prelude::*;

mod contexts;
mod state;

use contexts::*;

declare_id!("EQYhXLXF8QVpax7EgNRFzarwbEvCx39SnX3d1WxvXdSi");

#[program]
pub mod aequor {

    use super::*;

    #[instruction(discriminator = 0)]
    pub fn initialize(
        ctx: Context<InitializeAequors>,
        fee_authority: Pubkey,
        collect_fee_authority: Pubkey,
        reward_emissions_authority: Pubkey,
        protocol_fee_rate: u16,
    ) -> Result<()> {
        ctx.accounts.init(
            fee_authority,
            collect_fee_authority,
            reward_emissions_authority,
            protocol_fee_rate,
            &ctx.bumps,
        )
    }
}