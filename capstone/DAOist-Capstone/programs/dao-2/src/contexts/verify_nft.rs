use anchor_lang::prelude::*;
use mpl_token_metadata::instruction::verify_collection;
use solana_program::program::invoke_signed;
use anchor_spl::{
    metadata::{Metadata, MasterEditionAccount, MetadataAccount},
    token::Mint
};

use crate::state::DaoConfig;

#[derive(Accounts)]
pub struct VerifyNft<'info> {
    #[account(
        seeds=[b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    config: Account<'info, DaoConfig>,
    #[account(
        seeds=[b"mint", config.key().as_ref()],
        bump = config.mint_bump
    )]
    collection: Account<'info, Mint>,
    #[account(
        seeds = [
            b"metadata",
            mpl_program.key().as_ref(),
            collection.key().as_ref(),
            b"edition"
        ],
        seeds::program = mpl_program.key(),
        bump
    )]
    collection_edition: Box<Account<'info, MasterEditionAccount>>,
    #[account(
        seeds = [
            b"metadata",
            mpl_program.key().as_ref(),
            collection.key().as_ref(),
        ],
        seeds::program = mpl_program.key(),
        bump
    )]
    collection_metadata: Box<Account<'info, MetadataAccount>>,
    #[account(
        mint::decimals = 0,
    )]
    nft_mint: Account<'info, Mint>,
    #[account(
        mut,
        seeds = [
            b"metadata",
            mpl_program.key().as_ref(),
            nft_mint.key().as_ref()
        ],
        seeds::program = mpl_program.key(),
        bump
    )]
    nft_metadata: Box<Account<'info, MetadataAccount>>,
    #[account(
        seeds = [
            b"metadata",
            Metadata::id().as_ref(),
            nft_mint.key().as_ref(),
            b"edition"
        ],
        seeds::program = mpl_program.key(),
        bump
    )]
    nft_edition: Box<Account<'info, MasterEditionAccount>>,
    #[account(
        mut,
        seeds=[b"auth", config.key().as_ref()],
        bump = config.auth_bump
    )]
    ///CHECK: This is safe. It's just used to sign things
    auth: UncheckedAccount<'info>,
    mpl_program: Program<'info, Metadata>,
    #[account(mut)]
    signer: Signer<'info>
}

impl<'info> VerifyNft<'info> {
    pub fn verify_nft(&self) -> Result<()> {

        let program_id = self.mpl_program.key();
        let metadata = self.nft_metadata.key();
        let collection_authority = self.auth.key();
        let payer = self.signer.key();
        let collection_mint = self.collection.key();
        let collection_edition = self.collection_edition.key();
        let collection_metadata = self.collection_metadata.key();

        // Verify the collection in the metadata account
        let verify_collection_ix = verify_collection(
            program_id,
            metadata,
            collection_authority,
            payer,
            collection_mint,
            collection_metadata,
            collection_edition,
            None
        );

        let config_key = self.config.key();
        let auth_bump = self.config.auth_bump;

        let seeds = &[
            &b"auth"[..],
            &config_key.as_ref(),
            &[auth_bump]
        ];
        
        invoke_signed(
            &verify_collection_ix,
            &[
                self.nft_metadata.to_account_info(),
                self.auth.to_account_info(),
                self.signer.to_account_info(),
                self.collection.to_account_info(),
                self.collection_metadata.to_account_info(),
                self.collection_edition.to_account_info(),
            ],
            &[&seeds[..]]
        )?;

        Ok(())
    }
}