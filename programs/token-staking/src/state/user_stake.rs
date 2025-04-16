use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserStake {
    pub owner: Pubkey,
    pub pool_config: Pubkey,
    pub start_time: u64,
    pub end_time: u64,
    pub weight_multiplier: u64,
    // we need to know how much user have deposited (staked) as all goes to one vault and we do not emit any LP tokens
    pub amount: u64,
}

impl UserStake {
    pub const LEN: usize = 8 + Self::INIT_SPACE;

    pub const SEED_PREFIX: &'static [u8] = b"user_stake";

    // amount according to weight multiplier, need for reward calculation.
    // - This would be the amount of "LP" tokens if we would emit them (??)
    pub fn weighted_amount(&self) -> u64 {
        self.amount * self.weight_multiplier
    }
}