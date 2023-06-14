use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct StakeState {
    pub owner: Pubkey,
    pub amount: u64,
    pub accounts: u64,
    pub updated: u64,
    pub vault_bump: u8,
    pub auth_bump: u8,
    pub state_bump: u8,
}

impl StakeState {
    pub const LEN: usize = 8 + PUBKEY_L + 3 * U64_L + 3 * U8_L;

    pub fn init(
        &mut self,  
        owner: Pubkey,
        state_bump: u8,
        vault_bump: u8,
        auth_bump: u8
    ) -> Result<()> {
        self.owner = owner;
        self.amount = 0;
        self.accounts = 0;
        self.state_bump = state_bump;
        self.vault_bump = vault_bump;
        self.auth_bump = auth_bump;
        self.update()
    }

    pub fn stake(
        &mut self,
        amount: u64
    ) -> Result<()> {
        self.amount.checked_add(amount).ok_or(DaoError::Overflow)?;
        self.update()
    }

    pub fn unstake(
        &mut self,
        amount: u64
    ) -> Result<()> {
        self.check_accounts()?;
        self.check_slot()?; // Don't allow staking and unstaking in the same slot
        self.amount = self.amount.checked_sub(amount).ok_or(DaoError::Underflow)?;
        self.update()
    }

    pub fn add_account(&mut self) -> Result<()> {
        self.accounts = self.accounts.checked_add(1).ok_or(DaoError::Overflow)?;
        Ok(())
    }

    pub fn remove_account(&mut self) -> Result<()> {
        self.accounts = self.accounts.checked_sub(1).ok_or(DaoError::Underflow)?;
        Ok(())
    }

    // This might be convenient later, but comment out for now
    // pub fn remove_accounts(&mut self, amount: u64) -> Result<()> {
    //     self.accounts.checked_sub(amount).ok_or(DaoError::Underflow)
    // }

    pub fn update(&mut self) -> Result<()> {
        self.updated = Clock::get()?.slot;
        Ok(())
    }

    // Make sure the user doesn't unstake in the same slot
    pub fn check_slot(&mut self) -> Result<()> {
        require!(self.updated < Clock::get()?.slot, DaoError::InvalidSlot);
        Ok(())
    }    

    // Make sure the user doesn't have any open accounts
    pub fn check_accounts(&mut self) -> Result<()> {
        require!(self.accounts == 0, DaoError::AccountsOpen);
        Ok(())
    }

    // Ensure staked amount > 0
    pub fn check_stake(&mut self) -> Result<()> {
        require!(self.amount > 0, DaoError::InsufficientStake);
        Ok(())
    }
}