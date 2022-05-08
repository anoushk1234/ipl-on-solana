use anchor_lang::prelude::*;

#[account]
pub struct ArenaAccount {
    pub host: Pubkey,
    pub players: Vec<Pubkey>,
    pub aggregator_key: Pubkey,
    pub teams: Vec<Team>,
    pub bump: u8,
    pub arena_id: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Default, Clone)]
pub struct Team {
    pub nonce: u8,
    pub name: String,
}
#[account]
pub struct ArenaHostAccount {
    pub arena_count: u64,
    pub bump: u8,
}
#[account]
pub struct Player {
    pub bet_count: u64,
    pub bump: u8,
}

#[account]
pub struct Bet {
    pub wager_type: WagerTypes,
    pub aggregator_key: Pubkey,
    pub arena_key: Pubkey,
}
#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone)]
pub enum WagerTypes {
    MatchOutcome,
    InningRuns,
}
impl Default for WagerTypes {
    fn default() -> Self {
        WagerTypes::MatchOutcome
    }
}
