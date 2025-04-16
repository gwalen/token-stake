use std::borrow::{Borrow, BorrowMut};

use anchor_lang::prelude::*;

use crate::{instructions::stake_tokens::StakeTokens, state::{pool_config::{self, PoolConfig}, user_stake::{self, UserStake}}, utils::errors::StakeProgramErrors};

// Basis Points (Bips) its a 0.01% or 0.0001
pub const BIPS: u64 = 10_000;

pub fn handler(
    ctx: Context<StakeTokens>, 
    amount: u64, 
    lockup_period: u64
) -> Result<()> {
    let pool_config = ctx.accounts.pool_config.borrow();
    require!(amount > 0, StakeProgramErrors::AmountZero);
    require!(lockup_period >= pool_config.min_duration, StakeProgramErrors::LockupPeriodLessThanMin);
    require!(lockup_period <= pool_config.max_duration, StakeProgramErrors::LockupPeriodBiggerThaMax);

    let lock_period_start = Clock::get()?.unix_timestamp as u64;
    let user_weight_multiplier = calculate_user_weight_multiplier(pool_config, lockup_period);

    msg!("XXX - User multiplier: {}", user_weight_multiplier);

    let user_stake = ctx.accounts.user_stake.borrow_mut();
    user_stake.set_inner(UserStake {
        owner: ctx.accounts.user.key(),
        pool_config: ctx.accounts.pool_config.key(),
        stake_token_vault: ctx.accounts.stake_token_vault.key(),
        start_time: lock_period_start,
        end_time: lock_period_start + lockup_period,
        weight_multiplier: user_weight_multiplier
    });

    Ok(())
}

// Calculate user weight_multiplier in BIPS
fn calculate_user_weight_multiplier(pool_config: &PoolConfig, user_lockup_period: u64) -> u64 {
    // if lock_period == pool_config.min_duration => weight_multiplier = 1
    // if lock_period == pool_config.max_duration => weight_multiplier = pool_config.max_wight_multiplier
    // otherwise it is linear :
    // let pool_max_lock_period_duration = pool_config.max_duration - pool_config.min_duration
    // without bips :
    // weight_multiplier = 1 + (pool_config.max_wight_multiplier - 1) * lock_period / pool_max_lock_period_duration
    // reasoning: 
    // 1 is minimum amount, pool_config.max_wight_multiplier is max, if lockup_period > pool_config.min_duration
    // than we need to know how much of the distance between pool_config.max_wight_multiplier and 1 is covered by lock_period.
    // So it is lim 1 + (pool_config.max_wight_multiplier - 1) * lock_period / pool_max_lock_period_duration -> goes to pool_config.max_wight_multiplier
    // as lockup_period is increasing value up to pool_max_lock_period_duration 
    // Small adjustment: above will work if pool_config.min_duration is 0
    // we need to take into account how far is lockup_period in the distance between pool_config.min_duration - pool_config.max_duration
    // so introduce adjusted_lockup_period = lockup_period - pool_config.min_duration
    // finally we get:
    // weight_multiplier = 1 + (pool_config.max_wight_multiplier - 1) * adjusted_lock_period / pool_max_lock_period_duration

    if user_lockup_period == pool_config.min_duration {
        return 1;
    }
    if user_lockup_period == pool_config.max_duration {
        return pool_config.max_wight_multiplier;
    }

    let adjusted_lockup_period = user_lockup_period - pool_config.min_duration;
    let pool_max_lockup_period = pool_config.max_duration - pool_config.min_duration;
    // Below formula we need make in BIPS to make it non floating point so we multiply by 10_000 (BIPS)
    // weight_multiplier = 1 + (pool_config.max_wight_multiplier - 1) * adjusted_lock_period / pool_max_lock_period_duration
    let weight_multiplier = 
        1 * BIPS + 
        (pool_config.max_wight_multiplier - 1) * adjusted_lockup_period * BIPS / pool_max_lockup_period;
    // we also assume that pool_config.max_wight_multiplier is given as integer like 2x, 3x, 4x (and not floating values like 1.5x are available)
    // and not in BIPS, if it would be in BIPS than the last part of formula would change into:
    // (pool_config.max_wight_multiplier - BIPS) * adjusted_lockup_period / pool_max_lockup_period;   

    weight_multiplier
}