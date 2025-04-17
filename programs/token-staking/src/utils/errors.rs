use anchor_lang::prelude::*;

#[error_code]
#[derive(PartialEq)]
pub enum StakeProgramErrors {
    #[msg("Amount is zero")]
    AmountZero,
    #[msg("Lockup period is less than minimum value")]
    LockupPeriodLessThanMin,
    #[msg("Lockup period is bigger than max value")]
    LockupPeriodBiggerThaMax,
    #[msg("Invalid pool owner")]
    InvalidPoolOwner,
    #[msg("Invalid stake token")]
    InvalidStakeToken
}