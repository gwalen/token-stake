use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;

use instructions::pool_create::*;

declare_id!("14cNesu4Fnme8M6wqK5GMJWygsXYbQuae4KbyBp9aBNW");

#[program]
pub mod token_staking {
    use super::*;

    // TODO: remove
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    // *** pool owner instructions

    pub fn pool_create(ctx: Context<PoolCreate>) -> Result<()> {
        Ok(())
    }

    // pub fn pool_deposit(ctx: Context<PoolDeposit>) -> Result<()> {
    //     Ok(())
    // }

    // *** user instructions

    // pub fn stake(ctx: Context<StakeTokens>, amount: u64, lock_period: u64) -> Result<()> {
    //     Ok(())
    // }

    // pub fn unstake(ctx: Context<UnstakeTokens>) -> Result<()> {
    //     Ok(())
    // }

}


#[derive(Accounts)]
pub struct Initialize {}
