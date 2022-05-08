use crate::state::Team;
use anchor_lang::prelude::*;

#[event]
pub struct MatchResult {
    pub team: Team,
    pub nonce: u8,
    pub aggregator_key: Pubkey,
    pub arena_key: Pubkey,
}
