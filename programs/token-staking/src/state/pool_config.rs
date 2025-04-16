use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolConfig {
    pub owner: Pubkey,
    pub stake_token_mint: Pubkey,
    pub stake_token_vault: Pubkey, // TODO: remove not need to save it it is ATA : 
    pub min_duration: u64,
    pub max_duration: u64,
    pub max_wight_multiplier: u64,
    // total_amount -> this we do not need to store in variable as we can just check the amount of tokens in the vault
    // This is weighted amount of tokens in the pool (we need a separate variable for it as it is a sum of all users weighted amounts)
    pub total_weighted_amount: u64,
    pub reward_distributor: Pubkey
}

impl PoolConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub const SEED_PREFIX: &'static [u8] = b"pool_config";
}