use anchor_lang::prelude::*;
#[error_code]
pub enum CustomErrorCodes {
    #[msg("Invalid Aggregator value returned")]
    InvalidAggregatorValueReturned,
}
