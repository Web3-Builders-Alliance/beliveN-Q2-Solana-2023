use anchor_lang::prelude::*;

#[account]
pub struct MultiSig {
    /// The minimum signers required to execute
    pub min_signers: u8,
    /// The addresses of the signers
    pub keys: Vec<Pubkey>,
    /// The bump of multisig PDA
    pub multisig_bump: u8,
    /// The bump of multisig treasury PDA
    pub multisig_treasury_bump: u8
}

impl MultiSig {
    pub const FIXED_LEN: usize = 8 + 3 + 4;

    pub fn init(
        &mut self,
        min_signers: u8,
        keys: Vec<Pubkey>,
        multisig_bump: u8,
        multisig_treasury_bump: u8
    ) -> Result<()> {
        self.min_signers = min_signers;
        self.keys = keys;
        self.multisig_bump = multisig_bump;
        self.multisig_treasury_bump = multisig_treasury_bump;
        Ok(())
    }
}