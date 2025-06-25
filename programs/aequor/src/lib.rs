use anchor_lang::prelude::*;

declare_id!("EQYhXLXF8QVpax7EgNRFzarwbEvCx39SnX3d1WxvXdSi");

#[program]
pub mod aequor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
