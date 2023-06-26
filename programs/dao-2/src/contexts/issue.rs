use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{token::{Token, TokenAccount, MintTo, mint_to, Mint}, associated_token::AssociatedToken};

use crate::state::DaoConfig;

#[derive(Accounts)]
pub struct IssueTokens<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint,
        associated_token::authority = initializer
    )]
    initializer_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.treasury_bump
    )]
    treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    mint: Account<'info, Mint>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>
}

impl<'info> IssueTokens<'info> {
    pub fn deposit_sol(
        &self
    ) -> Result<()> {
        let accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.treasury.to_account_info()
        };

        let ctx = CpiContext::new(
            self.system_program.to_account_info(),
            accounts
        );

        transfer(ctx, self.config.issue_price)
    }

    pub fn issue_tokens(
        &self
    ) -> Result<()> {
        let accounts = MintTo {
            mint: self.mint.to_account_info(),
            to: self.initializer_ata.to_account_info(),
            authority: self.auth.to_account_info()
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

        mint_to(ctx, self.config.issue_amount)
    }
}
