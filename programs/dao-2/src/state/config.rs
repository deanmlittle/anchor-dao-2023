use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct DaoConfig {
    pub seed: u64,
    pub issue_price: u64,
    pub issue_amount: u64,
    pub proposal_fee: u64,
    pub max_supply: u64,
    pub min_quorum: u64,
    pub max_expiry: u64,
    pub proposal_count: u64,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub mint_bump: u8,
    pub treasury_bump: u8
}

impl DaoConfig {
    pub const LEN: usize = 8 + 6 * U64_L + 4 * U8_L;

    pub fn init(
        &mut self,
        seed: u64,
        issue_price: u64,
        issue_amount: u64,
        proposal_fee: u64,
        max_supply: u64,
        min_quorum: u64,
        max_expiry: u64,
        auth_bump: u8,
        config_bump: u8,
        mint_bump: u8,
        treasury_bump: u8        
    ) -> Result<()> {
        self.seed = seed;
        self.issue_price = issue_price;
        self.issue_amount = issue_amount;
        self.proposal_fee = proposal_fee;
        self.max_supply = max_supply;
        self.min_quorum = min_quorum;
        self.max_expiry = max_expiry;
        self.proposal_count = 0;
        self.auth_bump = auth_bump;
        self.config_bump = config_bump;
        self.mint_bump = mint_bump;
        self.treasury_bump = treasury_bump;
        Ok(())
    }

    pub fn add_proposal(&mut self, id: u64) -> Result<()> {
        self.proposal_count = self.proposal_count.checked_add(1).ok_or(DaoError::Overflow)?;
        require!(self.proposal_count == id, DaoError::InvalidProposalSeed);
        Ok(())
    }

    pub fn check_min_quorum(&self, quorum: u64) -> Result<()> {
        require!(self.min_quorum <= quorum, DaoError::InvalidQuorum);
        Ok(())
    }

    pub fn check_max_expiry(&self, expiry: u64) -> Result<()> {
        require!(self.max_expiry >= expiry, DaoError::InvalidExpiry);
        Ok(())
    }
}