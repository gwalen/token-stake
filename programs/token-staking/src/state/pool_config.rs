use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PoolConfig {
    pub owner: Pubkey,
    pub stake_token_ming: Pubkey,
    pub stake_token_vault: Pubkey,
    pub min_duration: u64,
    pub max_duration: u64,
    pub max_wight_multiplier: u64,
    #[max_len(10)]
    pub reward_distributions: Vec<Pubkey>
}

impl PoolConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub const SEED_PREFIX: &'static [u8] = b"pool_config";
}