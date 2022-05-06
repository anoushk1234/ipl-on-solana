use anchor_lang::prelude::*;
mod state;
use crate::state::*;
use switchboard_v2::AggregatorAccountData;
declare_id!("HDa6A4wLjEymb8Cv3UGeGBnFUNCUEMpoiVBbkTKAkfrt");

#[program]
pub mod sol_ipl {
    use super::*;

    pub fn create_arena(ctx: Context<CreateArena>) -> Result<()> {
        Ok(())
    }
    pub fn create_aggregator(ctx: Context<CreateAggregator>) -> Result<()> {
        let aggregator = &ctx.accounts.aggregator_feed;
        let val: f64 = AggregatorAccountData::new(aggregator)?
            .get_result()?
            .try_into()?;

        msg!("Current feed result is {}!", val);

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateArena<'info> {
    arena_account: Account<'info, ArenaAccount>,
}

#[derive(Accounts)]
pub struct CreateAggregator<'info> {
    pub authority: Signer<'info>,
    /// CHECK: field is unsafe
    pub aggregator_feed: AccountInfo<'info>, // pass aggregator key
}
