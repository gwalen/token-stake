use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserStake {
    pub owner: Pubkey,
    pub pool_config: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub weight_multiplier: u64
}

impl UserStake {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub const SEED_PREFIX: &'static [u8] = b"user_stake";
}