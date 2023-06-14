use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod constants;
mod state;
mod errors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dao_2 {
    use crate::{state::ProposalType, errors::DaoError};

    use super::*;

    // Instantiate a new DAO using the DAO2023 program
    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        issue_price: u64,
        issue_amount: u64,
        proposal_fee: u64,
        max_supply: u64,
        min_quorum: u64,
        max_expiry: u64
    ) -> Result<()> {
        ctx.accounts.init(seed, &ctx.bumps, issue_price, issue_amount, proposal_fee, max_supply, min_quorum, max_expiry)
    }

    // Handle token issuance
    pub fn issue_tokens(ctx: Context<IssueTokens>) -> Result<()> {
        ctx.accounts.deposit_sol()?;
        ctx.accounts.issue_tokens()
    }

    // Initialize a stake account for adding DAO tokens
    pub fn init_stake(ctx: Context<InitializeStake>) -> Result<()> {
        // Create a stake account
        ctx.accounts.init(&ctx.bumps)?;
    }

    // Stake DAO tokens
    pub fn stake_tokens(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Deposit tokens, add stake
        ctx.accounts.deposit_tokens(amount)?;
    }

    // Stake DAO tokens
    pub fn unstake_tokens(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Withdraw tokens, remove stake
        ctx.accounts.withdraw_tokens(amount)?;
    }

    // Create a proposal
    pub fn create_proposal(ctx: Context<CreateProposal>, id: u64, name: String, proposal: ProposalType, link: String, threshold: u64, amount: u64, data: Vec<u8>) -> Result<()> {
        // Pay a proposal fee to DAO treasury
        ctx.accounts.pay_proposal_fee()?;

        // Get proposal bump
        let bump = ctx.bumps.get("proposal").ok_or(DaoError::BumpError)?;

        // Ensure user has actually got tokens staked and create a new proposal
        ctx.accounts.create_proposal(
            id, 
            name, 
            proposal,
            link, 
            threshold, 
            amount,
            bump
        )
    }

    // Vote on a proposal
    pub fn vote(ctx: Context<Vote>, id: u64, votes: u64) -> Result<()> {
        // Increment votes for proposal, if quorum is reached, execute it.
        ctx.accounts.increment_vote(id)?;
        // Increment total number of proposals in the DAO
        ctx.accounts.vote(id)?;
    }

    // Close a voting position after a proposal has passed/expired
    pub fn close_vote(ctx: Context<CloseVote>, id: u64) -> Result<()> {
        // Decrement votes for proposal (if not expired or passed yet)
        ctx.accounts.close_vote(id)?;
        // Ensure user has actually got tokens staked and create a new proposal
        ctx.accounts.create_proposal(id, name, proposal, link, threshold)?;
    }
}