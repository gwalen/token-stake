use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::pool_config::PoolConfig;

#[derive(Accounts)]
pub struct PoolCreate<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = PoolConfig::LEN,
        seeds = [ 
            PoolConfig::SEED_PREFIX,
            &owner.key().to_bytes(),
            &stake_token_mint.key().to_bytes()
        ],
        bump
    )]
    pub pool_config: Account<'info, PoolConfig>,

    pub stake_token_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = stake_token_mint,
        associated_token::authority = pool_config
    )]
    pub stake_token_vault: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>

}