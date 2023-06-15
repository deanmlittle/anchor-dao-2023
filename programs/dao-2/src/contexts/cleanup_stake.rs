use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint, CloseAccount, close_account}, associated_token::AssociatedToken};

use crate::{state::{config::DaoConfig, StakeState}, errors::DaoError};

#[derive(Accounts)]
pub struct CleanupStake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.vault_bump,
        token::mint = mint,
        token::authority = stake_auth
    )]
    stake_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    stake_auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    mint: Account<'info, Mint>,
    #[account(
        mut,
        close = owner,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.state_bump
    )]
    stake_state: Account<'info, StakeState>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

impl<'info> CleanupStake<'info> {
    pub fn cleanup_stake(
        &mut self
    ) -> Result<()> {
        self.close_stake_ata()?;
        match self.stake_state.check_stake() {
            Ok(_) => err!(DaoError::InvalidStakeAmount),
            Err(_) => Ok(())
        }
    }

    pub fn close_stake_ata(
        &self
    ) -> Result<()> {
        let accounts = CloseAccount {
            account: self.stake_ata.to_account_info(),
            destination: self.owner.to_account_info(),
            authority: self.stake_auth.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().to_bytes()[..],
            &self.owner.key().to_bytes()[..],
            &[self.stake_state.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(), 
            accounts, 
            signer_seeds
        );

        close_account(ctx)
    }


}