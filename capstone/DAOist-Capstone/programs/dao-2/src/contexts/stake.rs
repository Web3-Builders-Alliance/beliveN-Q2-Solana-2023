use anchor_lang::prelude::*;
use anchor_spl::{token::{Token, TokenAccount, Transfer as TransferSpl, transfer as transfer_spl, Mint}, associated_token::AssociatedToken};

use crate::state::{config::DaoConfig, StakeState};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        init,
        payer = initializer,
        mint::authority = auth,
        mint::freeze_authority = auth,
        mint::decimals = 0
    )]
    nft: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = nft,
        associated_token::authority = initializer,
    )]
    token: Box<Account<'info, TokenAccount>>,
    #[account(
        seeds=[b"auth", config.key().as_ref(), owner.key().as_ref()],
        bump = stake_state.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    collection: Account<'info, Mint>,
    #[account(
        mut,
        seeds=[b"stake", config.seed.to_le_bytes().as_ref()],
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

impl<'info> Stake<'info> {
    pub fn deposit_tokens(
        &mut self,
        amount: u64
    ) -> Result<()> {
        self.stake_state.stake(amount)?;

        let accounts = TransferSpl {
            from: self.nft.to_account_info(),
            to: self.token.to_account_info(),
            authority: self.owner.to_account_info()
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            accounts
        );
        transfer_spl(ctx, amount)
    }

    pub fn withdraw_tokens(
        &self,
        amount: u64
    ) -> Result<()> {
        self.stake_state.unstake(amount)?;

        let accounts = TransferSpl {
            from: self.stake_ata.to_account_info(),
            to: self.owner_ata.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let seeds = &[
            &b"auth"[..],
            &self.config.key().to_bytes()[..],
            &self.auth.key().to_bytes()[..],
            &[self.stake_state.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds
        );

        transfer_spl(ctx, amount)
    }
}