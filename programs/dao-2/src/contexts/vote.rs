use anchor_lang::{prelude::*, system_program::Transfer};

use crate::{state::{config::DaoConfig, Proposal, StakeState, VoteState}, errors::DaoError};

#[derive(Accounts)]
pub struct Vote<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.state_bump
    )]
    stake_state: Account<'info, StakeState>,
    #[account(
        mut,
        seeds=[b"proposal", config.key().as_ref(), proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump,
    )]
    proposal: Account<'info, Proposal>,
    #[account(
        init,
        payer = owner,
        seeds=[b"vote", proposal.key().as_ref(), owner.key().as_ref()],
        bump,
        space = VoteState::LEN
    )]
    vote: Account<'info, VoteState>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    system_program: Program<'info, System>
}

impl<'info> Vote<'info> {
    pub fn vote(
        &mut self,
        amount: u64,
        bump: u8
    ) -> Result<()> {
        // Check proposal is open
        self.proposal.is_open()?;
        // Check proposal hasn't expired
        self.proposal.check_expiry()?;
        // Ensure vote amount > 0
        require!(amount > 0, DaoError::InvalidVoteAmount);
        // Add vote to proposal
        self.proposal.add_vote(amount)?;
        // Make sure user has staked
        self.stake_state.check_stake_amount(amount)?;
        // Add a vote account to the stake state
        self.stake_state.add_account()?;
        // Initialize vote
        self.vote.init(
            self.owner.key(),
            amount,
            bump
        )
    }
}