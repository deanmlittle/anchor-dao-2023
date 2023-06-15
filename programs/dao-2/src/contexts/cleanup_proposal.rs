use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};

use crate::{state::{config::DaoConfig, Proposal, ProposalType}, errors::DaoError};

#[derive(Accounts)]
pub struct CleanupProposal<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(mut)]
    payee: UncheckedAccount<'info>,
    #[account(
        mut,
        close = treasury,
        seeds=[b"proposal", config.key().as_ref(), proposal.id.to_le_bytes().as_ref()],
        bump = proposal.bump
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

impl<'info> CleanupProposal<'info> {
    pub fn cleanup_proposal(
        &mut self
    ) -> Result<()> {
        // Try finalize
        self.proposal.try_finalize();
        self.proposal.is_failed()?;
        Ok(())
    }

    pub fn execute_proposal(
        &mut self
    ) -> Result<()> {
        // Try finalize proposal
        self.proposal.try_finalize();
        // Check if the status is successful
        self.proposal.is_succeeded()?;
        match self.proposal.proposal {
            ProposalType::Bounty(payee, payout) => self.payout_bounty(payee, payout),
            ProposalType::Executable => self.execute_tx(),
            ProposalType::Vote => self.finalize_vote(),
        }
    }

    pub fn finalize_vote(&self) -> Result<()> {
        msg!("Vote result: {} / {}", self.proposal.votes, self.proposal.quorum);
        msg!("Vote has {:?}", self.proposal.result);
        Ok(())
    }

    pub fn payout_bounty(
        &self,
        payee: Pubkey,
        payout: u64
    ) -> Result<()> {
        require_keys_eq!(self.payee.key(), payee);

        let accounts = Transfer {
            from: self.treasury.to_account_info(),
            to: self.payee.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().to_bytes()[..],
            &[self.config.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer(ctx, payout)
    }

    pub fn execute_tx(
        &self
    ) -> Result<()> {
        // Instruction

        // Accounts

        // Signers
        
        Ok(())
    }
}