use crate::errors::AequorError;
use crate::state::{Aequor, TickArray, TICK_ARRAY_SIZE_USIZE};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(start_tick_index: i32)]
pub struct InitializeTickArray<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    pub aequor: Account<'info, Aequor>,

    #[account(
        init,
        payer = signer,
        seeds = [b"tick_array", aequor.key().as_ref(), start_tick_index.to_string().as_bytes()],
        bump,
        space = 8 + TickArray::INIT_SPACE
    )]
    pub tick_array: AccountLoader<'info, TickArray>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitializeTickArray<'info> {
    pub fn init(&self, start_tick_index: i32) -> Result<()> {
        require!(
            start_tick_index % (TICK_ARRAY_SIZE_USIZE as i32) == 0,
            AequorError::InvalidStartTickIndex
        );

        let mut tick_array = self.tick_array.load_init()?;

        tick_array.aequor = self.aequor.key();
        tick_array.start_tick_index = start_tick_index;
        Ok(())
    }
}
