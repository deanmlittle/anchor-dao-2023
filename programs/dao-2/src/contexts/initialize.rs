use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{errors::DaoError, state::DaoConfig};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        seeds=[b"auth", config.key().as_ref()],
        bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"mint", config.key().as_ref()],
        bump,
        mint::authority = auth,
        mint::decimals = 0
    )]
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds=[b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = DaoConfig::LEN
    )]
    config: Account<'info, DaoConfig>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self, 
        seed: u64,
        bumps: &BTreeMap<String, u8>,
        issue_price: u64,
        issue_amount: u64,
        proposal_fee: u64,
        max_supply: u64,
        min_quorum: u64,
        max_expiry: u64,
    ) -> Result<()> {
        let (
            auth_bump,
            config_bump,
            mint_bump,
            treasury_bump
        ) = (
            *bumps.get("auth").ok_or(DaoError::BumpError)?,
            *bumps.get("config").ok_or(DaoError::BumpError)?,
            *bumps.get("mint").ok_or(DaoError::BumpError)?,
            *bumps.get("treasury").ok_or(DaoError::BumpError)?,
        );

        self.config.init(
            seed,
            issue_price,
            issue_amount,
            proposal_fee,
            max_supply,
            min_quorum,
            max_expiry,
            auth_bump,
            config_bump,
            mint_bump,
            treasury_bump
        )
    }
}