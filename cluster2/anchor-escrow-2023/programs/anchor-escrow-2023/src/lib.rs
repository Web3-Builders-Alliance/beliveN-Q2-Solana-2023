
use anchor_lang::prelude::*;

declare_id!("3tWcxiouPwVkF8eiWWFPbxV8vayMVaR8BYMRFcz2zFK3");

mod errors;
use errors::EscrowError;
mod structs;
mod contexts;
use contexts::*;

#[program]
pub mod anchor_escrow_2023 {
    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        offer_amount: u64,
        expiry: u64,
    ) -> Result<()> {
        // Set up our escrow variables
        require!(expiry.gt(&10_000u64), EscrowError::MaxExpiryExceeded);
        let escrow = &mut ctx.accounts.escrow;
        escrow.maker = *ctx.accounts.maker.key;
        escrow.maker_token = *ctx.accounts.maker_token.to_account_info().key;
        escrow.taker_token = *ctx.accounts.taker_token.to_account_info().key;
        escrow.seed = seed;
        escrow.offer_amount = offer_amount;
        escrow.auth_bump = *ctx.bumps.get("auth").ok_or(EscrowError::AuthBumpError)?;
        escrow.vault_bump = *ctx.bumps.get("vault").ok_or(EscrowError::VaultBumpError)?;
        escrow.escrow_bump = *ctx.bumps.get("escrow").ok_or(EscrowError::EscrowBumpError)?;

        if expiry > 0 {

            escrow.expiry = Clock::get()?.slot.checked_add(expiry).ok_or(EscrowError::ExpiryError)?; //error here 
        } 
        else {
            escrow.expiry = 0;           
        }
        // Transfer maker tokens to the vault
        ctx.accounts.transfer_to_vault(deposit_amount)
    }

    // Cancel and refund escrow to the maker
    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }

    // Allow maker to change the token and offer amount of the escrow
    pub fn update(
        ctx: Context<Update>,
        offer_amount: u64,
        expiry: u64
    ) -> Result<()> {
        let escrow = &mut ctx.accounts.escrow;
        escrow.taker_token = *ctx.accounts.new_taker_token.to_account_info().key;
        escrow.offer_amount = offer_amount;
        if expiry > 0 {       

            escrow.expiry = Clock::get()?.slot.checked_add(expiry).ok_or(EscrowError::ExpiryError)?;   
        } 
        else {

            escrow.expiry = 0;
            
        }
        Ok(())
    }

    // Allow taker to accept the escrow
    pub fn take(ctx: Context<Take>) -> Result<()> {

        let escrow = &mut ctx.accounts.escrow;

        let slot = Clock::get()?.slot;

        if escrow.expiry > 0
        {
        require!(escrow.expiry > slot, EscrowError::Expired);
    }

        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.empty_vault_to_taker()?;
        ctx.accounts.close_vault()
    }
}