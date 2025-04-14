use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::state::pool_config::PoolConfig;

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,

    pub stake_token_mint: Account<'info, Mint>,

    #[account(
        constraint 
    )]
    pub pool_config: Account<'info, PoolConfig>,
}