use anchor_lang::prelude::*;
mod errors;
mod events;
mod state;
use crate::errors::CustomErrorCodes::*;
use crate::events::*;
use crate::state::*;
use switchboard_v2::AggregatorAccountData;
declare_id!("HDa6A4wLjEymb8Cv3UGeGBnFUNCUEMpoiVBbkTKAkfrt");

#[program]
pub mod sol_ipl {
    use super::*;
    pub fn initialize_arena_host(ctx: Context<InitializeArenaHost>) -> Result<()> {
        ctx.accounts.arena_host_account.arena_count = 0;
        ctx.accounts.arena_host_account.bump = *ctx.bumps.get("arena_host_account").unwrap();
        Ok(())
    }
    pub fn create_arena(
        ctx: Context<CreateArena>,
        players: Vec<Pubkey>,
        aggregator_key: Pubkey,
        team_a: Team,
        team_b: Team,
    ) -> Result<()> {
        ctx.accounts.arena_account.bump = *ctx.bumps.get("arena_account").unwrap();
        ctx.accounts.arena_account.aggregator_key = aggregator_key;
        ctx.accounts.arena_account.players = players;
        ctx.accounts.arena_account.teams = vec![team_a, team_b];
        ctx.accounts.arena_account.arena_id = ctx
            .accounts
            .arena_host_account
            .arena_count
            .checked_add(1)
            .unwrap();
        Ok(())
    }
    pub fn get_result(ctx: Context<GetResult>) -> Result<()> {
        let aggregator = &ctx.accounts.aggregator_feed;
        let val: u64 = AggregatorAccountData::new(aggregator)?
            .get_result()?
            .try_into()?;

        msg!("Current feed result is {}!", val);
        require!(
            val == 0 || val == 1 || val == 2,
            Err(InvalidAggregatorValueReturned)
        );
        let result_team: Team = *ctx
            .accounts
            .arena_account
            .teams
            .iter()
            .find(|&t| t.nonce == val as u8)
            .unwrap();
        emit!(MatchResult {
            team: result_team,
            aggregator_key: ctx.accounts.aggregator_feed.key(),
            nonce: val as u8,
            arena_key: ctx.accounts.arena_account.key()
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct CreateArena<'info> {
    #[account(
        init,
        payer=host,
        seeds = [b"arena-account-key",host.key().as_ref(),[arena_host_account.arena_count as u8].as_ref()], bump,
        space=9000)]
    pub arena_account: Account<'info, ArenaAccount>,
    #[account(mut)]
    pub host: Signer<'info>,
    #[account(mut,seeds = [b"arena-host-key",host.key().as_ref()
    ], bump=arena_host_account.bump)]
    pub arena_host_account: Account<'info, ArenaHostAccount>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetResult<'info> {
    pub authority: Signer<'info>,
    /// CHECK: field is unsafe
    pub aggregator_feed: AccountInfo<'info>, // pass aggregator key
    #[account(mut,seeds = [b"arena-account-key",arena_account.host.key().as_ref()], bump=arena_account.bump)]
    pub arena_account: Account<'info, ArenaAccount>,
}

#[derive(Accounts)]
pub struct InitializeArenaHost<'info> {
    #[account(
        init,
        seeds = [b"arena-host-key",authority.key().as_ref()], bump,payer = authority,
        space=9000)]
    pub arena_host_account: Account<'info, ArenaHostAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
