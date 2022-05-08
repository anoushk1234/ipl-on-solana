mod errors;
mod events;
mod state;
mod structures;
use crate::errors::CustomErrorCodes::*;
use crate::events::*;
use crate::state::*;
use crate::structures::*;

use anchor_lang::prelude::*;
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
    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        ctx.accounts.player_account.bet_count = 0;
        ctx.accounts.player_account.bump = *ctx.bumps.get("arena_host_account").unwrap();
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
    pub fn place_bet(ctx: Context<PlaceBet>) -> Result<()> {
        ctx.accounts
            .player_account
            .bet_count
            .checked_add(1)
            .unwrap();
        ctx.accounts.bet_account.wager_type = WagerTypes::MatchOutcome;
        ctx.accounts.bet_account.aggregator_key = ctx.accounts.arena_account.aggregator_key;
        ctx.accounts.bet_account.arena_key = ctx.accounts.arena_account.key();

        Ok(())
    }
}
