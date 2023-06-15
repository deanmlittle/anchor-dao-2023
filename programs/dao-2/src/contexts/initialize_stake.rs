use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Mint}, associated_token::AssociatedToken};

use crate::{state::{config::DaoConfig, StakeState}, errors::DaoError};

#[derive(Accounts)]
pub struct InitializeStake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    owner_ata: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        seeds = [b"vault", config.key().as_ref(), owner.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = stake_auth
    )]
    stake_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", config.key().as_ref(), owner.key().as_ref()],
        bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    stake_auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = owner,
        seeds=[b"stake", config.key().as_ref(), owner.key().as_ref()],
        bump,
        space = StakeState::LEN
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

impl<'info> InitializeStake<'info> {
    pub fn init(
        &self,
        bumps: &BTreeMap<String, u8>
    ) -> Result<()> {
        self.stake_state.init(
            self.owner.key(),
            *bumps.get("stake_state").ok_or(DaoError::BumpError)?,
            *bumps.get("stake_ata").ok_or(DaoError::BumpError)?,
            *bumps.get("stake_auth").ok_or(DaoError::BumpError)?
        )
    }
}