use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct RewardDistributorConfig {
    pub pool_config: Pubkey,
    pub reward_token_mint: Pubkey,
    // in sec
    pub emission_rate: u64
}

impl RewardDistributorConfig {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub const SEED_PREFIX: &'static [u8] = b"reward_config";
}