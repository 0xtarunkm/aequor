use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        mint_to, set_authority, spl_token_2022::instruction::AuthorityType, Mint, MintTo,
        SetAuthority, TokenAccount, TokenInterface,
    },
};

use crate::state::{Aequor, Position};

#[derive(Accounts)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        mint::decimals = 0,
        mint::authority = aequor,
        mint::token_program = token_program
    )]
    pub position_mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = signer,
        associated_token::mint = position_mint,
        associated_token::authority = owner
    )]
    pub position_token_account: InterfaceAccount<'info, TokenAccount>,

    /// CHECK: safe, the account that will be the owner of the position can be arbitrary
    pub owner: UncheckedAccount<'info>,
    pub aequor: Account<'info, Aequor>,

    #[account(
        init,
        payer = signer,
        space = 8 + Position::INIT_SPACE,
        seeds = [b"position".as_ref(), position_mint.key().as_ref()],
        bump
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> OpenPosition<'info> {
    pub fn init(
        &mut self,
        tick_lower_index: i32,
        tick_upper_index: i32,
        bumps: &OpenPositionBumps,
    ) -> Result<()> {
        self.position.set_inner(Position {
            aequor: self.aequor.key(),
            position_mint: self.position_mint.key(),
            liquidity: 0,
            tick_lower_index,
            tick_upper_index,
            fee_growth_checkpoint_a: 0,
            fee_owed_a: 0,
            fee_growth_checkpoint_b: 0,
            fee_owed_b: 0,
            position_bump: bumps.position,
        });

        self.mint_token()?;
        self.revoke_mint_authority()
    }

    fn mint_token(&self) -> Result<()> {
        let accounts = MintTo {
            mint: self.position_mint.to_account_info(),
            to: self.position_token_account.to_account_info(),
            authority: self.aequor.to_account_info(),
        };

        let seeds = &[
            b"aequor".as_ref(),
            self.aequor.aequors_config.as_ref(),
            self.aequor.mint_a.as_ref(),
            self.aequor.mint_b.as_ref(),
            self.aequor.fee_tier_index_seed.as_ref(),
            &[self.aequor.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        mint_to(cpi_ctx, 1)
    }

    fn revoke_mint_authority(&self) -> Result<()> {
        let accounts = SetAuthority {
            account_or_mint: self.position_mint.to_account_info(),
            current_authority: self.aequor.to_account_info(),
        };

        let seeds = &[
            b"aequor".as_ref(),
            self.aequor.aequors_config.as_ref(),
            self.aequor.mint_a.as_ref(),
            self.aequor.mint_b.as_ref(),
            self.aequor.fee_tier_index_seed.as_ref(),
            &[self.aequor.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        set_authority(cpi_ctx, AuthorityType::MintTokens, None)
    }
}
