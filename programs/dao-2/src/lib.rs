use anchor_lang::prelude::*;

mod contexts;
use contexts::*;
mod constants;
mod state;
mod errors;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod dao_2 {
    use crate::{errors::DaoError, state::ProposalType};

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
        ctx.accounts.init(&ctx.bumps)
    }

    // Close a stake account when you're done with it
    pub fn close_stake(ctx: Context<CloseStake>) -> Result<()> {
        // Create a stake account
        ctx.accounts.close(&ctx.bumps)
    }

    // Stake DAO tokens
    pub fn stake_tokens(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Deposit tokens, add stake
        ctx.accounts.deposit_tokens(amount)
    }

    // Stake DAO tokens
    pub fn unstake_tokens(ctx: Context<Stake>, amount: u64) -> Result<()> {
        // Withdraw tokens, remove stake
        ctx.accounts.withdraw_tokens(amount)
    }

    // Create a proposal
    pub fn create_proposal(
        ctx: Context<CreateProposal>, 
        id: u64, 
        name: String, 
        gist: String, 
        proposal: ProposalType, 
        threshold: u64, 
        amount: u64, 
        data: Vec<u8>
    ) -> Result<()> {
        // Pay a proposal fee to DAO treasury
        ctx.accounts.pay_proposal_fee()?;

        // Ensure user has actually got tokens staked and create a new proposal
        ctx.accounts.create_proposal(
            id, 
            name, 
            gist,
            proposal,
            threshold, 
            amount,
            *ctx.bumps.get("proposal").ok_or(DaoError::BumpError)?
        )
    }

    // Cleanup a proposal
    pub fn cleanup_proposal(
        ctx: Context<CreateProposal>, 
    ) -> Result<()> {
        // Pay a proposal fee to DAO treasury
        ctx.accounts.cleanup_proposal()
    }

     // Cleanup a proposal
     pub fn execute_proposal(
        ctx: Context<CreateProposal>, 
    ) -> Result<()> {
        // Pay a proposal fee to DAO treasury
        ctx.accounts.cleanup_proposal()
    }

    // Vote on a proposal
    pub fn vote(ctx: Context<Vote>, amount: u64) -> Result<()> {
        // Increment total number of votes in the proposal
        ctx.accounts.vote(amount, *ctx.bumps.get("vote").ok_or(DaoError::BumpError)?)
    }

    // Close a voting position after a proposal has passed/expired
    pub fn cleanup_vote(ctx: Context<Unvote>) -> Result<()> {
        // Decrement votes for user
        ctx.accounts.cleanup_vote()
    }

    // Close a voting position in an active proposal
    pub fn remove_vote(ctx: Context<Unvote>) -> Result<()> {
        // Decrement votes for user and proposal
        ctx.accounts.remove_vote()
    }
}