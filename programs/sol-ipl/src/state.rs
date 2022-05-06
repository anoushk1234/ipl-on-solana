use anchor_lang::prelude::*;

#[account]
pub struct ArenaAccount {
    pub host: Pubkey,
    pub players: Vec<Pubkey>,
}
