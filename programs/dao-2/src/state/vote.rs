use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub owner: Pubkey,
    pub amount: u64,
    pub bump: u8
}

impl VoteState {
    pub const LEN: usize = 8 + PUBKEY_L + U64_L + U8_L;

    pub fn init(
        &mut self,
        owner: Pubkey,
        amount: u64,
        bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.amount = amount;
        self.bump = bump;
        Ok(())
    }
}