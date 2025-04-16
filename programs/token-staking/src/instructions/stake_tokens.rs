use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::state::{pool_config::PoolConfig, user_stake::UserStake};

#[derive(Accounts)]
pub struct StakeTokens<'info> {
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    /// CHECK: pool_owner account used to seed generation (it is a publicly available account)
    pub pool_owner: UncheckedAccount<'info>,

    pub stake_token_mint: Account<'info, Mint>,

    #[account(
        seeds = [
            PoolConfig::SEED_PREFIX,
            &pool_owner.key().to_bytes(),
            &stake_token_mint.key().to_bytes()
        ],
        bump

    )]
    pub pool_config: Account<'info, PoolConfig>,

    // NOTE: init here gives us gurantee that there will be just one such user_stake account, 
    // which also means that user can stake() tokens to given poll only once (!)
    // TODO:2: think how to do if we want to have allow user to stake() several times.
    //         This would mean updating the lockup-period new total user stake tokens amount (?) and probably we need to pay out the rewards first (?)
    #[account(
        init,
        payer = user,
        space = UserStake::LEN,
        seeds = [
            UserStake::SEED_PREFIX,
            &user.key().to_bytes(),
            &stake_token_mint.key().to_bytes()
        ],
        bump
    )]
    pub user_stake: Account<'info, UserStake>,

    #[account(
        mut,
        associated_token::mint = stake_token_mint, 
        associated_token::authority = pool_config
    )]
    pub stake_token_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = stake_token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: Account<'info, TokenAccount>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>
}