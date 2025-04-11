use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PoolCreate<'info> {

    #[account(mut)]
    pub owner: Signer<'info>
}