use anchor_lang::prelude::*;

use crate::state::{AequorConfig, FeeTier};

#[derive(Accounts)]
pub struct InitializeFeeTier<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + FeeTier::INIT_SPACE
    )]
    pub fee_tier: Account<'info, FeeTier>,

    pub aequors_config: Account<'info, AequorConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeFeeTier<'info> {
    pub fn init(
        &mut self,
        tick_spacing: u16,
        default_fee_rate: u16,
    ) -> Result<()> {
        self.fee_tier.set_inner(FeeTier {
            aequors_config: self.aequors_config.key(),
            tick_spacing,
            default_fee_rate,
        });

        Ok(())
    }
} 