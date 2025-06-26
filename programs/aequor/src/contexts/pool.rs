use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Aequor, AequorConfig, FeeTier};
use crate::util::PoolInitialized;

#[derive(Accounts)]
#[instruction(tick_spacing: u16)]
pub struct InitializeAequorPool<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        seeds = [b"lp", aequor.key().as_ref()],
        payer = signer,
        bump,
        mint::decimals = 6,
        mint::authority = aequor,
        mint::token_program = token_program
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_a,
        associated_token::authority = aequor
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_b,
        associated_token::authority = aequor
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    pub aequors_config: Account<'info, AequorConfig>,

    #[account(
        init,
        payer = signer,
        seeds = [
            b"aequor".as_ref(),
            aequors_config.key().as_ref(),
            mint_a.key().as_ref(),
            mint_b.key().as_ref(),
            tick_spacing.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + Aequor::INIT_SPACE
    )]
    pub aequor: Account<'info, Aequor>,

    #[account(
        constraint = fee_tier.tick_spacing == tick_spacing
    )]
    pub fee_tier: Account<'info, FeeTier>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAequorPool<'info> {
    pub fn init(
        &mut self,
        tick_spacing: u16,
        initial_sqrt_price: u128,
        bumps: &InitializeAequorPoolBumps,
    ) -> Result<()> {
        let mint_a = self.mint_a.key();
        let mint_b = self.mint_b.key();
        let mint_lp = self.mint_lp.key();

        let fee_tier_index_seed = tick_spacing;

        let default_fee_rate = self.fee_tier.default_fee_rate;

        self.aequor.set_inner(Aequor {
            aequors_config: self.aequors_config.key(),
            tick_spacing,
            fee_tier_index_seed: fee_tier_index_seed.to_le_bytes(),
            fee_rate: default_fee_rate,
            protocol_fee_rate: self.aequors_config.protocol_fee_rate,
            liquidity: 0,
            sqrt_price: initial_sqrt_price,
            tick_current_index: 0,
            protocol_fee_a: 0,
            protocol_fee_b: 0,
            mint_a,
            vault_a: self.vault_a.key(),
            fee_growth_global_a: 0,
            mint_b,
            mint_lp,
            vault_b: self.vault_b.key(),
            fee_growth_global_b: 0,
            bump: bumps.aequor,
        });

        emit!(PoolInitialized {
            aequor: self.aequor.key(),
            aequors_config: self.aequors_config.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            tick_spacing,
            mint_a_ata: self.vault_a.key(),
            mint_b_ata: self.vault_b.key(),
            decimals_a: self.mint_a.decimals,
            decimals_b: self.mint_b.decimals,
            initial_sqrt_price,
        });

        Ok(())
    }
}
