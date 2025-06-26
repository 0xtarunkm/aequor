use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked, MintTo, mint_to}};

use crate::state::{Aequor, AequorConfig, Position};

#[derive(Accounts)]
pub struct Liquidity<'info> {
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
        mint::token_program = token_program
    )]
    pub mint_lp: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = aequor
    )]
    pub vault_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = aequor
    )]
    pub vault_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = signer
    )]
    pub user_ata_a: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = signer
    )]
    pub user_ata_b: InterfaceAccount<'info, TokenAccount>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint_lp,
        associated_token::authority = signer
    )]
    pub user_ata_lp: InterfaceAccount<'info, TokenAccount>,

    pub aequors_config: Account<'info, AequorConfig>,
    #[account(mut)]
    pub aequor: Account<'info, Aequor>,
    
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>
}

impl<'info> Liquidity<'info> {
    pub fn add(&mut self, amount: u64) -> Result<()> {
        self.deposit(amount, true)?;
        self.deposit(amount, false)?;
        self.mint_lp_tokens(amount)
    }

    fn deposit(&mut self, amount: u64, is_a: bool) -> Result<()> {
        let (from, to, mint, decimals) = match is_a {
            true => (self.user_ata_a.to_account_info(), self.vault_a.to_account_info(), self.mint_a.to_account_info(), self.mint_a.decimals),
            false => (self.user_ata_b.to_account_info(), self.vault_b.to_account_info(), self.mint_b.to_account_info(), self.mint_b.decimals)
        };

        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.signer.to_account_info()
        };

        let ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(ctx, amount, decimals)
    }

    fn mint_lp_tokens(
        &mut self,
        amount: u64
    ) -> Result<()> {
        let accounts = MintTo {
            authority: self.aequor.to_account_info(),
            to: self.user_ata_lp.to_account_info(),
            mint: self.mint_lp.to_account_info()
        };

        let config_key = self.aequors_config.key();
        let mint_a_key = self.mint_a.key();
        let mint_b_key = self.mint_b.key();

        let seeds = &[
            b"aequor",
            config_key.as_ref(),
            mint_a_key.as_ref(),
            mint_b_key.as_ref(),
            self.aequor.fee_tier_index_seed.as_ref(),
            &[self.aequor.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        mint_to(ctx, amount)
    }
}