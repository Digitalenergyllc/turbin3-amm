use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub seed: u64,                 // Seed to be able to create different pools / configs
    pub authority: Option<Pubkey>, // If we want an authority to lock the config account
    pub mint_x: Pubkey,            // Token X
    pub mint_y: Pubkey,            // Token Y
    pub fee: u16,                  // Swap fee in basis points
    pub treasury_fee: u16,         // Protocol fee in basis points, sent to the treasury
    pub locked: bool,              // If the pool is locked
    pub config_bump: u8,           // Bump seed for the config account
    pub lp_bump: u8,               // Bump seed for the LP token
}
// storing the bump seeds in the config account saves compute cost allowing later instructions to write bump = config.lp_bump instead of having to call find_program_address again.
// signing as Config PDA, the signer seeds have to include the bump seeds, having it stored means it's always on hand.
