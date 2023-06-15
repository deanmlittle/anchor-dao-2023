use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct Proposal {
    pub id: u64, // A unique ID. Can be sequential or random.
    pub name: String, // A proposal name
    pub gist: String, // 72 bytes (39 bytes + / + 32 char ID)
    pub proposal: ProposalType,
    pub result: ProposalStatus,
    pub quorum: u64,
    pub votes: u64,
    pub expiry: u64,
    pub bump: u8,
}

impl Proposal {
    pub const LEN: usize = 8 + 32 + 72 + ENUM_L * 2 + U8_L * 2 + 3 * U64_L + U8_L;
    pub fn init(
        &mut self,
        id: u64,
        name: String,
        gist: String,
        proposal: ProposalType,
        quorum: u64,
        expiry: u64,
        bump: u8  
    ) -> Result<()> {
        require!(name.len() < 33, DaoError::InvalidName);
        require!(gist.len() < 73, DaoError::InvalidGist);
        self.id = id;
        self.proposal = proposal;
        self.name = name;
        self.gist = gist;
        self.result = ProposalStatus::Open;
        self.quorum = quorum;
        self.votes = 0;
        self.bump = bump;
        self.expiry = Clock::get()?.slot.checked_add(expiry).ok_or(DaoError::Overflow)?;
        Ok(())
    }

    pub fn try_finalize(
        &mut self
    ) {
        if self.votes >= self.quorum && self.check_expiry().is_ok() {
            self.result = ProposalStatus::Succeeded
        } else if self.votes < self.quorum && self.check_expiry().is_err() {
            self.result = ProposalStatus::Failed
        }
    }

    pub fn check_expiry(
        &mut self
    ) -> Result<()> {
        require!(Clock::get()?.slot < self.expiry, DaoError::Expired);
        Ok(())
    }

    pub fn is_open(
        &mut self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Open, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn is_succeeded(
        &self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Succeeded, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn is_failed(
        &self
    ) -> Result<()> {
        require!(self.result == ProposalStatus::Failed, DaoError::InvalidProposalStatus);
        Ok(())
    }

    pub fn add_vote(
        &mut self,
        amount: u64
    ) -> Result<()> {
        self.votes = self.votes.checked_add(amount).ok_or(DaoError::Overflow)?;
        self.try_finalize();
        Ok(())
    }

    pub fn remove_vote(
        &mut self,
        amount: u64
    ) -> Result<()> {
        self.votes = self.votes.checked_sub(amount).ok_or(DaoError::Underflow)?;
        Ok(())
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, PartialEq, Eq)]
pub enum ProposalType {
    Bounty(Pubkey, u64), // Pay an address some amount of SOL
    Executable, // Sign some kind of instruction(s) with an accounts struct, etc
    Vote // We just want to know what people think. No money involved
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProposalStatus {
    Open,
    Succeeded,
    Failed
}