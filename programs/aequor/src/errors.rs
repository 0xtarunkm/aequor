use anchor_lang::prelude::*;

#[error_code]
pub enum AequorError {
    #[msg("Not a valid start index for tick")]
    InvalidStartTickIndex
}