use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
  #[msg("Already initialized")]
  AlreadyInitialized,
  #[msg("Not initialized")]
  NotInitialized,
}