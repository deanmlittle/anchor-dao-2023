use anchor_lang::prelude::error_code;

#[error_code]
pub enum DaoError {
    #[msg("Default Error")]
    DefaultError,
    #[msg("Bump Error")]
    BumpError,
    #[msg("Overflow")]
    Overflow,
    #[msg("Underflow")]
    Underflow,
    #[msg("You can't unstake with open accounts")]
    AccountsOpen,
    #[msg("Proposal expired")]
    Expired,
    #[msg("Invalid slot")]
    InvalidSlot,
    #[msg("Insufficient stake")]
    InsufficientStake,
    #[msg("Invalid name")]
    InvalidName,
    #[msg("Invalid gist")]
    InvalidGist,
    #[msg("Invalid proposal seed")]
    InvalidProposalSeed,
    #[msg("Invalid quorum")]
    InvalidQuorum,
    #[msg("Invalid expiry")]
    InvalidExpiry,
    #[msg("Proposal closed")]
    ProposalClosed
}