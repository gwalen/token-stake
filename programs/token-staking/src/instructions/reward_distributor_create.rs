use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{state::{pool_config::PoolConfig, reward_distributor_config::RewardDistributorConfig}, utils::errors::StakeProgramErrors};


#[derive(Accounts)]
pub struct CreateRewardDistributor<'info> {

    #[account(
        mut,
        constraint = pool_owner.key() == pool_config.owner.key() @ StakeProgramErrors::InvalidPoolOwner
    )]
    pub pool_owner: Signer<'info>,

    pub reward_token_mint: Account<'info, Mint>,

    #[account(
        mut, // we will set the reward distributor account in pool config
        seeds = [
            PoolConfig::SEED_PREFIX,
            &pool_config.key().to_bytes(),
            &pool_config.key().to_bytes()
        ],
        bump // TODO: save bump for the PoolConfig and other account to minimize the CU amount used
    )]
    pub pool_config: Account<'info, PoolConfig>,

    #[account(
        init,
        payer = pool_owner,
        space = RewardDistributorConfig::INIT_SPACE,
        seeds = [
            RewardDistributorConfig::SEED_PREFIX,
            
        ],
        bump
    )]
    pub reward_distributor_config: Account<'info, RewardDistributorConfig>,

    pub system_program: Program<'info, System>,
    

}