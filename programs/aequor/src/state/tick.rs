use anchor_lang::prelude::*;

pub const TICK_ARRAY_SIZE_USIZE: usize = 88;

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(InitSpace)]
pub struct Tick {
    pub initialized: bool,
    pub liquidity_net: i128,
    pub liquidity_gross: u128,

    pub fee_growth_outside_a: u128,
    pub fee_growth_outside_b: u128,
}

#[account(zero_copy(unsafe))]
#[repr(C, packed)]
#[derive(InitSpace)]
pub struct TickArray {
    pub start_tick_index: i32,
    pub ticks: [Tick; TICK_ARRAY_SIZE_USIZE],
    pub aequor: Pubkey,
}
