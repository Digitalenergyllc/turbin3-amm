use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"lp", config.key.as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub mint_lp: Account<'info, Mint>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"treasury", config.key().as_ref(), mint_x.key().as_ref()],
        bump,
        token::mint = mint_x,
        token::authority = config,
    )]
    pub treasury_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"treasury", config.key().as_ref(), mint_y.key().as_ref()],
        bump,
        token::mint = mint_y,
        token::authority = config,
    )]
    pub treasury_y: Box<Account<'info, TokenAccount>>,
    #[account(
        init,
        payer = initializer,
        seeds = [b"config", seed.to_le_bytes().as_ref()],
        bump,
        space = Config::DISCRIMINATOR.len() + Config::INIT_SPACE,
    )]
    pub config: Account<'info, Config>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        seed: u64,
        fee: u16,
        treasury_fee: u16,
        authority: Option<Pubkey>,
        bumps: InitializeBumps,
    ) -> Result<()> {
        // LP fee + treasury fee together must stay below 100%
        require!(
            (fee as u32) + (treasury_fee as u32) < 10_000,
            AmmError::InvalidFee
        );

        self.config.set_inner(Config {
            seed,
            authority,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee,
            treasury_fee,
            locked: false,
            config_bump: bumps.config,
            lp_bump: bumps.mint_lp,
        });

        Ok(())
    }
}

// this creates the Config PDA (authority = config), LP mint PDAs, and two vault ATAs owned by Config PDA.
// PDA has no private seeds so no person can sign for it. The program owns the PDA and can sign for it using the seeds and bump. The vault ATAs are owned by the Config PDA, so only the program can sign for them as well.
// The only way to get its signature is for the program that owns it to call invoke_signed with the right seeds, which is what CpiContext::new_with_signer does
// new LP tokens can only appear through code paths you wrote, mainly deposit
// the program code itself is the gatekeeper
// any instruction that signs with the config seeds could mint, so I control which ones do.
