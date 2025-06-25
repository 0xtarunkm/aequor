use anchor_lang::prelude::*;

use crate::state::AequorConfig;

#[derive(Accounts)]
pub struct InitializeAequors<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + AequorConfig::INIT_SPACE
    )]
    pub config: Account<'info, AequorConfig>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAequors<'info> {
    pub fn init(
        &mut self,
        fee_authority: Pubkey,
        collect_fee_authority: Pubkey,
        protocol_fee_rate: u16,
    ) -> Result<()> {
        self.config.set_inner(AequorConfig {
            fee_authority,
            collect_fee_authority,
            protocol_fee_rate,
        });

        Ok(())
    }
}
