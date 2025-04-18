use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::{state::pool_config::PoolConfig, utils::errors::StakeProgramErrors};

#[derive(Accounts)]
pub struct UnstakeUserTokens<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        constraint = stake_token_mint.key() == pool_config.stake_token_mint.key() @ StakeProgramErrors::InvalidStakeToken
    )]
    pub stake_token_mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            PoolConfig::SEED_PREFIX,
            &pool_config.owner.key().to_bytes(),
            &pool_config.stake_token_mint.key().to_bytes(),
        ],
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,

    // TODO: add ata constrains 
    pub stake_token_vault: Account<'info, TokenAccount>,

    // TODO: add ata constrains 
    pub stake_token_user_ata: Account<'info, TokenAccount>, 

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>
}