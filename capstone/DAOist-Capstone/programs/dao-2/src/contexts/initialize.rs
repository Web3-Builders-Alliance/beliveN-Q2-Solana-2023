use std::collections::BTreeMap;

use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint, Token, TokenAccount, MintTo, mint_to},
    associated_token::AssociatedToken, metadata::Metadata
};
use mpl_token_metadata::{
    instruction::{create_metadata_accounts_v3, create_master_edition_v3}
};

use solana_program::program::invoke_signed;

use crate::{errors::DaoError, state::{DaoConfig, MultiSig}};

#[derive(Accounts)]
#[instruction(seed: u64, multisig_keys: Vec<Pubkey>)]
pub struct Initialize<'info> {
    #[account(mut)]
    initializer: Signer<'info>,
    #[account(
        mut,
        seeds=[b"auth", config.key().as_ref()],
        bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    #[account(
        seeds=[b"treasury", config.key().as_ref()],
        bump
    )]
    main_treasury: SystemAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"mint", config.key().as_ref()],
        bump,
        mint::authority = auth,
        mint::freeze_authority = auth,
        mint::decimals = 0
    )]
    collection: Box<Account<'info, Mint>>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = collection,
        associated_token::authority = auth,
    )]
    token: Box<Account<'info, TokenAccount>>,
    /// This account is initialized in the ix
    #[account(mut)]
    metadata: UncheckedAccount<'info>,
    // This account is initialized in the ix
    #[account(mut)]
    edition: UncheckedAccount<'info>,
    #[account(
        init,
        payer = initializer,
        seeds=[b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = DaoConfig::LEN
    )]
    config: Account<'info, DaoConfig>,
    #[account(
        init,
        payer = initializer,
        space = MultiSig::FIXED_LEN + multisig_keys.len() * 32,
        seeds=[b"multisig", config.key().as_ref()],
        bump,

    )]
    multi_sig: Account<'info, MultiSig>,
    #[account(
        seeds=[b"multisig-treasury", multi_sig.key().as_ref()],
        bump,
    )]
    multisig_treasury: SystemAccount<'info>,
    mpl_program: Program<'info, Metadata>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    rent_program: Sysvar<'info, Rent>
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self, 
        seed: u64,
        multisig_keys: Vec<Pubkey>,
        min_signers: u8,
        bumps: &BTreeMap<String, u8>,
        issue_price: u64,
        proposal_fee: u64,
        max_supply: u64,
        min_quorum: u64,
        min_threshold: u64,
        max_expiry: u64,
        min_stake:u64,
        min_choices:u8,
        prevoting_period: u64,
        multisig_share: u8,
        dev_treasury_share: u8,
        ops_treasury_share: u8,
        name: String,
        symbol: String,
        uri: String
    ) -> Result<()> {
        let (
            auth_bump,
            config_bump,
            mint_bump,
            main_treasury_bump,
            multisig_bump,
            multisig_treasury_bump
        ) = (
            *bumps.get("auth").ok_or(DaoError::BumpError)?,
            *bumps.get("config").ok_or(DaoError::BumpError)?,
            *bumps.get("collection").ok_or(DaoError::BumpError)?,
            *bumps.get("main_treasury").ok_or(DaoError::BumpError)?,
            *bumps.get("multi_sig").ok_or(DaoError::BumpError)?,
            *bumps.get("multisig_treasury").ok_or(DaoError::BumpError)?

        );

        // Validate Multisig Params
        let keys_len = multisig_keys.len() as u8;
        let mut keys = multisig_keys;
        keys.sort(); 
        keys.dedup();

        require_gt!(keys_len, 0, DaoError::InvalidKeysLen);
        require_gte!(u8::MAX, keys_len, DaoError::InvalidKeysLen);
        require_gte!(keys_len, min_signers, DaoError::InvalidSigners);
        require_gt!(min_signers, 0, DaoError::InvalidSigners);

        // Validate the treasury division
        let total_share: u8 = 100;
        let mut main_treasury_share = (total_share).checked_sub(multisig_share).ok_or(DaoError::Overflow)?;
        main_treasury_share = (main_treasury_share).checked_sub(dev_treasury_share).ok_or(DaoError::Overflow)?;
        main_treasury_share = (main_treasury_share).checked_sub(ops_treasury_share).ok_or(DaoError::Overflow)?;

        // Authority Seeds
        let config_key = self.config.key();

        let auth_seeds = &[
            &b"auth"[..],
            &config_key.as_ref(),
            &[auth_bump]
        ];

        // Mint NFT Token
        mint_to(self.mint_nft_ctx().with_signer(&[&auth_seeds[..]]), 1)?;

        let program_id = self.mpl_program.key();
        let metadata = self.metadata.key();
        let edition = self.edition.key();
        let mint = self.collection.key();
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
            None,
            0,
            false,
            true,
            None,
            None,
            None,
        );

        invoke_signed(
            &create_metadata_ix,
            &[
                self.metadata.to_account_info(),
                self.collection.to_account_info(),
                self.auth.to_account_info(),
                self.initializer.to_account_info(),
                self.auth.to_account_info(),
                self.system_program.to_account_info(),
                self.rent_program.to_account_info()
            ],
            &[&auth_seeds[..]]
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
                self.collection.to_account_info(),
                self.auth.to_account_info(),
                self.auth.to_account_info(),
                self.initializer.to_account_info(),
                self.metadata.to_account_info(),
                self.token_program.to_account_info(),
                self.system_program.to_account_info(),
                self.rent_program.to_account_info()
            ],
            &[&auth_seeds[..]]
        )?;

        // Initialize Multisig
        self.multi_sig.init(min_signers, keys, multisig_bump, multisig_treasury_bump)?;

        // Initialize DAO Config
        self.config.init(
            seed,
            issue_price,
            proposal_fee,
            max_supply,
            min_quorum,
            min_threshold,
            max_expiry,
            min_stake,
            min_choices,
            prevoting_period,
            auth_bump,
            config_bump,
            mint_bump,
            main_treasury_bump,
            multisig_share,
            main_treasury_share,
            dev_treasury_share,
            ops_treasury_share
        )
    }

    pub fn mint_nft_ctx(&self) -> CpiContext<'_, '_, '_, 'info, MintTo<'info>> {
        let cpi_accounts = MintTo {
            mint: self.collection.to_account_info(),
            to: self.token.to_account_info(),
            authority: self.auth.to_account_info()
        };

        let cpi_program = self.token_program.to_account_info();

        CpiContext::new(cpi_program, cpi_accounts)
    }
}