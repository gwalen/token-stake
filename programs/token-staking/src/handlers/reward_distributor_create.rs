use std::borrow::BorrowMut;

use anchor_lang::prelude::*;

use crate::{instructions::reward_distributor_create::RewardDistributorCreate, state::reward_distributor_config::RewardDistributorConfig, utils::errors::StakeProgramErrors};

pub fn handler(
    ctx: Context<RewardDistributorCreate>,
    emission_rate: u64
) -> Result<()> {
    require!(emission_rate > 0, StakeProgramErrors::EmissionRateZero);
    
    let pool_config = ctx.accounts.pool_config.borrow_mut();
    require!(
        pool_config.reward_distributor == Pubkey::default(), 
        StakeProgramErrors::RedeclarationOfRewardDistributor
    );

    let reward_distributor_config = ctx.accounts.reward_distributor_config.borrow_mut();

    reward_distributor_config.set_inner(RewardDistributorConfig {
        pool_config: pool_config.key(),
        reward_token_mint: ctx.accounts.reward_token_mint.key(),
        emission_rate
    });

    pool_config.reward_distributor = ctx.accounts.reward_distributor_config.key();

    Ok(())
}