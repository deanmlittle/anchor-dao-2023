use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct VoteState {
    pub amount: u64,
    pub updated: u64,
    pub vote_bump: u8,
}

pub enum VoteType {

}

impl VoteState {
    pub const LEN: usize = 8 + PUBKEY_L + 2 * U64_L + ENUM_L + 2 * U8_L;

    pub fn init(
        &mut self,
        amount: u64,
        vote_bump: u8,
    ) -> Result<()> {
        self.owner = owner;
        self.vote = vote;
        self.amount = amount;
        self.vote_bump = vote_bump;
        self.update()
    }

    pub fn update(&mut self) -> Result<()> {
        self.updated = Clock::get()?.slot;
        Ok(())
    }
}