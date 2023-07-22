use crate::{constants::*, errors::DaoError};
use anchor_lang::prelude::*;

#[account]
pub struct DaoConfig {
    pub seed: u64,
    pub issue_price: u64,
    pub proposal_fee: u64,
    pub max_supply: u64,
    pub min_quorum: u64,
    pub min_threshold: u64,
    pub max_expiry: u64,
    pub min_stake: u64,
    pub min_choices: u8,
    pub prevoting_period: u64,
    pub proposal_count: u64,
    pub auth_bump: u8,
    pub config_bump: u8,
    pub mint_bump: u8,
    pub treasury_bump: u8,
    pub main_treasury_bump: u8,
    pub multisig_share: u8,
    pub main_treasury_share: u8,
    pub dev_treasury_share: u8,
    pub ops_treasury_share: u8
}

impl DaoConfig {
    pub const LEN: usize = 8 + 10 * U64_L + 9 * U8_L;

    pub fn init(
        &mut self,
        seed: u64,
        issue_price: u64,
        proposal_fee: u64,
        max_supply: u64,
        min_quorum: u64,
        min_threshold: u64,
        max_expiry: u64,
        min_stake: u64,
        min_choices: u8,
        prevoting_period: u64,
        auth_bump: u8,
        config_bump: u8,
        mint_bump: u8,
        main_treasury_bump: u8,
        multisig_share: u8,
        main_treasury_share: u8,
        dev_treasury_share: u8,
        ops_treasury_share: u8             
    ) -> Result<()> {
        self.seed = seed;
        self.issue_price = issue_price;
        self.proposal_fee = proposal_fee;
        self.max_supply = max_supply;
        self.min_quorum = min_quorum;
        self.min_threshold = min_threshold;
        self.max_expiry = max_expiry;
        self.min_stake = min_stake;
        self.min_choices = min_choices;
        self.prevoting_period = prevoting_period;
        self.proposal_count = 0;
        self.auth_bump = auth_bump;
        self.config_bump = config_bump;
        self.mint_bump = mint_bump;
        self.main_treasury_bump = main_treasury_bump;
        self.multisig_share = multisig_share;
        self.main_treasury_share = main_treasury_share;
        self.dev_treasury_share = dev_treasury_share;
        self.ops_treasury_share = ops_treasury_share;
        Ok(())
    }

    pub fn check_min_choices(&self, min_choices: u8) -> Result<()>{
        require!(self.min_choices <= min_choices, DaoError::InvalidChoicesAmount);
        Ok(())
    }

    pub fn check_min_stake(&self, min_stake: u64) -> Result<()> {
        require!(self.min_stake <= min_stake, DaoError::InvalidStakeAmount);
        Ok(())
    }


    
    pub fn check_min_threshold(&self, threshold: u64) -> Result<()> {
        require!(self.min_threshold <= threshold, DaoError::InvalidThreshold);
        Ok(())
    }


    pub fn add_proposal(&mut self, id: u64) -> Result<()> {
        self.proposal_count = self.proposal_count.checked_add(1).ok_or(DaoError::Overflow)?;
        require!(self.proposal_count == id, DaoError::InvalidProposalSeed);
        Ok(())
    }

    pub fn check_min_quorum(&self, quorum: u64) -> Result<()> {
        require!(self.min_quorum <= quorum, DaoError::InvalidQuorum);
        Ok(())
    }

    pub fn check_max_expiry(&self, expiry: u64) -> Result<()> {
        require!(self.max_expiry >= expiry, DaoError::InvalidExpiry);
        Ok(())
    }
}