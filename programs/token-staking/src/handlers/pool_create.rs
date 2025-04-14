use std::borrow::BorrowMut;

use anchor_lang::prelude::*;

use crate::{instructions::pool_create::PoolCreate, state::pool_config::PoolConfig};


pub fn handle(
    ctx: Context<PoolCreate>,
    min_duration: u64,
    max_duration: u64,
    max_wight_multiplier: u64,
) -> Result<()> {
    let pool_config = ctx.accounts.pool_config.borrow_mut(); 

    pool_config.set_inner(PoolConfig {
        owner: ctx.accounts.owner.key(),
        stake_token_mint: ctx.accounts.stake_token_mint.key(),
        stake_token_vault: ctx.accounts.stake_token_vault.key(),
        min_duration,
        max_duration,
        max_wight_multiplier,
        reward_distributions: Vec::new()
    });

    Ok(())
}
