use anchor_lang::prelude::*;

declare_id!("4TLhT3e1bn8UcSg6qQScKhWi1LwEy52yy7cFn6J1rt7S");

#[program]
pub mod wba_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {

        ctx.accounts.vault_state.score = 0;
        ctx.accounts.vault_state.vault_bump = *ctx.bumps.get("vault_auth").unwrap();
        ctx.accounts.vault_state.auth_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.vault_state.owner = *ctx.accounts.owner.key;

      /*   let vault_state = &mut ctx.accounts.vault_state;
        vault_state.score = 0; */
        
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize <'info>
{
    #[account(mut)]
    pub owner : Signer <'info>,
    #[account(init, payer=owner, space=Vault::LEN)]
    pub vault_state : Account <'info, Vault>,
    #[account(seeds = [b"auth", vault_state.key().as_ref()], bump)]
    /// CHECK
    pub vault_auth: UncheckedAccount<'info>,
    #[account(seeds = [b"vault", vault_auth.key().as_ref()], bump)]
    pub vault : SystemAccount <'info>,
    pub system_program: Program <'info, System>
}

#[account]
pub struct Vault
{
    owner: Pubkey,
    auth_bump: u8,
    vault_bump: u8,
    score: u8,

}

impl Vault {
    const LEN:usize = 50 ;

}
