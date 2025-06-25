use anchor_lang::prelude::*;

mod contexts;
mod state;
mod util;

use contexts::*;

declare_id!("EQYhXLXF8QVpax7EgNRFzarwbEvCx39SnX3d1WxvXdSi");

#[program]
pub mod aequor {

    use super::*;

    /// Initializes the Aequor protocol with the specified authorities and fee configuration.
    ///
    /// # Arguments
    /// * `fee_authority` - The public key of the authority that can modify fee parameters
    /// * `collect_fee_authority` - The public key of the authority that can collect protocol fees
    /// * `reward_emissions_authority` - The public key of the authority that can modify reward emissions
    /// * `protocol_fee_rate` - The initial protocol fee rate (basis points)
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok(()) if initialization is successful
    #[instruction(discriminator = 0)]
    pub fn initialize_config(
        ctx: Context<InitializeAequors>,
        fee_authority: Pubkey,
        collect_fee_authority: Pubkey,
        protocol_fee_rate: u16,
    ) -> Result<()> {
        ctx.accounts.init(
            fee_authority,
            collect_fee_authority,
            protocol_fee_rate
        )
    }

    /// Initializes a new Aequor liquidity pool with the specified parameters.
    /// 
    /// # Arguments
    /// * `tick_spacing` - The spacing between ticks for this pool, determines price granularity
    /// * `initial_sqrt_price` - The initial square root price of the pool (Q64.64 fixed-point)
    /// 
    /// # Returns
    /// * `Result<()>` - Returns Ok(()) if pool initialization is successful
    #[instruction(discriminator = 1)]
    pub fn initialize_pool(
        ctx: Context<InitializeAequorPool>,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        ctx.accounts.init(tick_spacing, initial_sqrt_price)
    }
}
