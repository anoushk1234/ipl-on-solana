use anchor_lang::prelude::*;

use crate::state::*;

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
    #[account(mut,seeds = [b"arena-account-key",arena_account.host.key().as_ref(),&[arena_account.arena_id as u8].as_ref()], bump=arena_account.bump)]
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
#[derive(Accounts)]
pub struct InitializePlayer<'info> {
    #[account(
        init,
        seeds = [b"player",authority.key().as_ref()], bump,payer = authority,
        space=9000)]
    pub player_account: Account<'info, Player>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        init,
        seeds = [b"bet",player.key().as_ref(),&[]], bump,payer = player,
        space=9000)]
    pub bet_account: Account<'info, Bet>,

    #[account(mut,seeds = [b"arena-account-key",arena_account.host.key().as_ref(),&[arena_account.arena_id as u8].as_ref()], bump=arena_account.bump)]
    pub arena_account: Account<'info, ArenaAccount>,

    #[account(mut,seeds = [b"player",player.key().as_ref()],bump=player_account.bump)]
    pub player_account: Account<'info, Player>,

    #[account(mut)]
    pub player: Signer<'info>,

    #[account(mut,seeds = [b"arena-host-key",arena_account.host.key().as_ref()], bump=arena_host_account.bump)]
    pub arena_host_account: Account<'info, ArenaHostAccount>,
    pub system_program: Program<'info, System>,
}
