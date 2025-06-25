use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{state::{Aequor, AequorConfig, FeeTier}, util::PoolInitialized};

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
        payer = signer,
        token::mint = mint_a,
        token::authority = aequor
    )]
    pub mint_a_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        token::mint = mint_b,
        token::authority = aequor
    )]
    pub mint_b_ata: InterfaceAccount<'info, TokenAccount>,

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
        has_one = aequors_config, 
        constraint = fee_tier.tick_spacing == tick_spacing
    )]
    pub fee_tier: Account<'info, FeeTier>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeAequorPool<'info> {
    pub fn init(&mut self, tick_spacing: u16, initial_sqrt_price: u128) -> Result<()> {
        let mint_a = self.mint_a.key();
        let mint_b = self.mint_b.key();

        let fee_tier_index = tick_spacing;

        let default_fee_rate = self.fee_tier.default_fee_rate;

        self.aequor.set_inner(Aequor { 
            aequors_config: self.aequors_config.key(),
            tick_spacing,
            fee_tier_index_seed: fee_tier_index.to_le_bytes(),
            fee_rate: default_fee_rate,
            protocol_fee_rate: self.aequors_config.protocol_fee_rate,
            liquidity: 0,
            sqrt_price: initial_sqrt_price,
            tick_current_index: 0,
            protocol_fee_a: 0,
            protocol_fee_b: 0,
            mint_a,
            vault_a: self.mint_a_ata.key(),
            fee_growth_global_a: 0,
            mint_b,
            vault_b: self.mint_b_ata.key(),
            fee_growth_global_b: 0
        });

        emit!(PoolInitialized {
            aequor: self.aequor.key(),
            aequors_config: self.aequors_config.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            tick_spacing,
            mint_a_ata: self.mint_a_ata.key(),
            mint_b_ata: self.mint_b_ata.key(),
            decimals_a: self.mint_a.decimals,
            decimals_b: self.mint_b.decimals,
            initial_sqrt_price
        });

        Ok(())
    }
}