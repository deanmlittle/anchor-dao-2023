use anchor_lang::prelude::*;

use crate::{state::{config::DaoConfig, Proposal, StakeState, VoteState}, errors::DaoError};

#[derive(Accounts)]
pub struct Unvote<'info> {
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
        mut,
        close = treasury,
        seeds=[b"vote", proposal.key().as_ref(), owner.key().as_ref()],
        bump = vote.bump
    )]
    vote: Account<'info, VoteState>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    system_program: Program<'info, System>
}

impl<'info> Unvote<'info> {
    pub fn cleanup_vote(
        &mut self
    ) -> Result<()> {
        if self.proposal.is_open().is_ok() && self.proposal.check_expiry().is_ok() {
            return err!(DaoError::InvalidProposalStatus);
        }
        // Remove a vote account to the stake state
        self.stake_state.remove_account()
    }

    pub fn remove_vote(
        &mut self
    ) -> Result<()> {
        self.proposal.is_open()?;
        self.proposal.check_expiry()?;
        self.proposal.remove_vote(self.vote.amount)?;
        self.stake_state.remove_account()
    }
}