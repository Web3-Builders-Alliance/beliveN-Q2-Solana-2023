use anchor_lang::prelude::*;
use crate::{DaoError, state::TreasuryType};

pub fn validate_treasury(treasury: String) -> Result<()> {
    let _treasury_type = match treasury.as_str() {
        "treasury" => TreasuryType::Main,
        "dev-treasury" => TreasuryType::Dev,
        "ops-treasury" => TreasuryType::Ops,
        _ => { return err!(DaoError::InvalidTreasury); }
    };
    Ok(())
}