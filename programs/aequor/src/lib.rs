use anchor_lang::prelude::*;

mod contexts;
mod errors;
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
        ctx.accounts
            .init(fee_authority, collect_fee_authority, protocol_fee_rate)
    }

    /// Initializes a new fee tier with the specified parameters.
    ///
    /// # Arguments
    /// * `tick_spacing` - The spacing between ticks for pools using this fee tier
    /// * `default_fee_rate` - The default fee rate for pools using this fee tier (basis points)
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok(()) if fee tier initialization is successful
    #[instruction(discriminator = 1)]
    pub fn initialize_fee_tier(
        ctx: Context<InitializeFeeTier>,
        tick_spacing: u16,
        default_fee_rate: u16,
    ) -> Result<()> {
        ctx.accounts.init(tick_spacing, default_fee_rate)
    }

    /// Initializes a new Aequor liquidity pool with the specified parameters.
    ///
    /// # Arguments
    /// * `tick_spacing` - The spacing between ticks for this pool, determines price granularity
    /// * `initial_sqrt_price` - The initial square root price of the pool (Q64.64 fixed-point)
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok(()) if pool initialization is successful
    #[instruction(discriminator = 2)]
    pub fn initialize_pool(
        ctx: Context<InitializeAequorPool>,
        tick_spacing: u16,
        initial_sqrt_price: u128,
    ) -> Result<()> {
        ctx.accounts
            .init(tick_spacing, initial_sqrt_price, &ctx.bumps)
    }

    /// Initializes a new tick array for storing tick data in the pool.
    ///
    /// # Arguments
    /// * `start_tick_index` - The starting tick index for this array. Must be a multiple of TICK_ARRAY_SIZE (88).
    ///                        For example: -88, 0, 88, 176, etc.
    ///
    /// # Returns
    /// * `Result<()>` - Returns Ok(()) if tick array initialization is successful
    ///
    /// # Errors
    /// * `InvalidStartTickIndex` - If the start_tick_index is not a multiple of TICK_ARRAY_SIZE
    #[instruction(discriminator = 3)]
    pub fn initialize_tick_array(
        ctx: Context<InitializeTickArray>,
        start_tick_index: i32,
    ) -> Result<()> {
        ctx.accounts.init(start_tick_index)
    }

}
