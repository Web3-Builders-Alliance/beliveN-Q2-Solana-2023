use anchor_lang::{prelude::*, system_program::{Transfer, transfer}};
use anchor_spl::{
    token::{Token, TokenAccount, MintTo, mint_to, Mint}, 
    associated_token::AssociatedToken,
    metadata::Metadata
};
use mpl_token_metadata::{
    state::{Creator, Collection},
    instruction::{create_metadata_accounts_v3, create_master_edition_v3}
};

use solana_program::program::invoke_signed;

use crate::state::{DaoConfig, MultiSig};

#[derive(Accounts)]
pub struct IssueTokens<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds=[b"treasury", config.key().as_ref()],
        bump = config.main_treasury_bump
    )]
    main_treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"dev-treasury", config.key().as_ref()],
        bump
    )]
    dev_treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"ops-treasury", config.key().as_ref()],
        bump
    )]
    ops_treasury: SystemAccount<'info>,
    #[account(
        mut,
        seeds=[b"multisig-treasury", multi_sig.key().as_ref()],
        bump = multi_sig.multisig_treasury_bump
    )]
    multisig_treasury: SystemAccount<'info>,
    #[account(
        seeds=[b"multisig", config.key().as_ref()],
        bump = multi_sig.multisig_bump,
    )]
    multi_sig: Account<'info, MultiSig>,
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
    /// This account is initialized in the ix
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    // This account is initialized in the ix
    #[account(mut)]
    edition: UncheckedAccount<'info>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    collection: Account<'info, Mint>,
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    mpl_program: Program<'info, Metadata>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    rent_program: Sysvar<'info, Rent>
}

impl<'info> IssueTokens<'info> {
    pub fn deposit_sol(
        &self
    ) -> Result<()> {
        let issue_price = self.config.issue_price;
        let mut remaining_amount = issue_price;
        let main_share = self.config.main_treasury_share as u64;
        let dev_share = self.config.dev_treasury_share as u64;
        let ops_share = self.config.ops_treasury_share as u64;

        // Transfer to main treasury
        let main_share = (issue_price * main_share) / 100;
        remaining_amount = remaining_amount - main_share;

        let main_accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.main_treasury.to_account_info()
        };

        let main_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            main_accounts
        );

        transfer(main_ctx, main_share)?;

        // Transfer to dev treasury
        let dev_share = (issue_price * dev_share) / 100;
        remaining_amount = remaining_amount - dev_share;

        let dev_accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.dev_treasury.to_account_info()
        };

        let dev_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            dev_accounts
        );

        transfer(dev_ctx, dev_share)?;

        // Transfer to ops treasury
        let ops_share = (issue_price * ops_share) / 100;
        remaining_amount = remaining_amount - ops_share;

        let ops_accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.ops_treasury.to_account_info()
        };

        let ops_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            ops_accounts
        );

        transfer(ops_ctx, ops_share)?;

        // Transfer to multisig treasury
        let mulsig_accounts = Transfer {
            from: self.initializer.to_account_info(),
            to: self.multisig_treasury.to_account_info()
        };

        let mulsig_ctx = CpiContext::new(
            self.system_program.to_account_info(),
            mulsig_accounts
        );

        transfer(mulsig_ctx, remaining_amount)
    }

    pub fn issue_tokens(
        &self,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        let config_key = self.config.key();
        let auth_bump = self.config.auth_bump;

        let seeds = &[
            &b"auth"[..],
            &config_key.as_ref(),
            &[auth_bump]
        ];

        let accounts = MintTo {
            mint: self.nft.to_account_info(),
            to: self.token.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let ctx = CpiContext::new(
            self.token_program.to_account_info(),
            accounts,
        );

        // Mint NFT token
        mint_to(ctx.with_signer(&[&seeds[..]]), 1)?;

        let program_id = self.mpl_program.key();
        let metadata = self.metadata.key();
        let edition = self.edition.key();
        let mint = self.nft.key();
        let mint_authority = self.auth.key();
        let payer = self.initializer.key();

        // Create Metadata
        let create_metadata_ix = create_metadata_accounts_v3(
            program_id,
            metadata,
            mint,
            mint_authority,
            payer,
            mint_authority,
            name,
            symbol,
            uri,
            Some(vec![
                Creator {
                    address: mint_authority,
                    verified: true,
                    share: 100
                }
            ]),
            500,
            false,
            true,
            Some(Collection { verified: false, key: self.collection.key() }),
            None,
            None,
        );

        invoke_signed(
            &create_metadata_ix,
            &[
                self.metadata.to_account_info(),
                self.nft.to_account_info(),
                self.auth.to_account_info(),
                self.initializer.to_account_info(),
                self.auth.to_account_info(),
                self.system_program.to_account_info(),
                self.rent_program.to_account_info()
            ],
            &[&seeds[..]]
        )?;


        // Create Master Edition
        let create_master_edition_ix = create_master_edition_v3(
            program_id,
            edition,
            mint,
            mint_authority,
            mint_authority,
            metadata,
            payer,
            Some(0)
        );

        invoke_signed(
            &create_master_edition_ix,
            &[
                self.edition.to_account_info(),
                self.nft.to_account_info(),
                self.auth.to_account_info(),
                self.auth.to_account_info(),
                self.initializer.to_account_info(),
                self.metadata.to_account_info(),
                self.token_program.to_account_info(),
                self.system_program.to_account_info(),
                self.rent_program.to_account_info()
            ],
            &[&seeds[..]]
        )?;

        Ok(())
    }
}
