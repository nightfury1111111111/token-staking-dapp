use anchor_lang::prelude::*;

#[account]
pub struct GlobalPool {
    pub admin: Pubkey,
}

#[account]
pub struct UserState {
    pub amount: u64,
    pub mint: Pubkey
}