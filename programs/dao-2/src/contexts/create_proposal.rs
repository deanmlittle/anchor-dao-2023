use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{state::{config::DaoConfig, Proposal, StakeState, ProposalType}, errors::DaoError};

#[derive(Accounts)]
#[instruction(id: u64)]
pub struct CreateProposal<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.state_bump
    )]
    stake_state: Account<'info, StakeState>,
    #[account(
        init,
        payer = owner,
        seeds=[b"proposal", config.key().as_ref(), id.to_le_bytes().as_ref()],
        bump,
        space = Proposal::LEN
    )]
    proposal: Account<'info, Proposal>,
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

impl<'info> CreateProposal<'info> {

    pub fn create_proposal(
        &mut self,
        id: u64,
        name: String,
        gist: String,
        proposal: ProposalType,
        quorum: u64,
        expiry: u64,
        bump: u8
    ) -> Result<()> {
        // Make sure user has staked
        self.stake_state.check_stake()?;
        // Add to account count
        self.stake_state.add_account()?;
        // Check ID and add proposal
        self.config.add_proposal(id)?;
        // Check minimum quorum
        self.config.check_min_quorum(quorum)?;
        // Check max expiry
        self.config.check_max_expiry(expiry)?;
        // Initialize the proposal
        self.proposal.init(
            id,
            name, // A proposal name
            gist, // 72 bytes (39 bytes + / + 32 byte ID)
            proposal,
            quorum,
            expiry,
            bump
        )
    }

    pub fn pay_proposal_fee(
        &mut self
    ) -> Result<()> {
        let accounts = Transfer {
            from: self.owner.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            accounts
        );

        transfer(ctx, self.config.proposal_fee)
    }
}