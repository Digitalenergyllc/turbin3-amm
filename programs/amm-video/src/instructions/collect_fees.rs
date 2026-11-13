use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::AmmError, state::Config};

#[derive(Accounts)]
pub struct CollectFees<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    pub mint_x: Box<Account<'info, Mint>>,
    pub mint_y: Box<Account<'info, Mint>>,
    #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config", config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info, Config>,
    #[account(
        mut,
        seeds = [b"treasury", config.key().as_ref(), mint_x.key().as_ref()],
        bump,
    )]
    pub treasury_x: Box<Account<'info, TokenAccount>>,
    #[account(
        mut,
        seeds = [b"treasury", config.key().as_ref(), mint_y.key().as_ref()],
        bump,
    )]
    pub treasury_y: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_x,
        associated_token::authority = authority,
    )]
    pub authority_x: Box<Account<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = authority,
        associated_token::mint = mint_y,
        associated_token::authority = authority,
    )]
    pub authority_y: Box<Account<'info, TokenAccount>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> CollectFees<'info> {
    pub fn collect_fees(&mut self) -> Result<()> {
        require!(
            self.config.authority == Some(self.authority.key()),
            AmmError::InvalidAuthority
        );

        let x = self.treasury_x.amount;
        let y = self.treasury_y.amount;
        require!(x > 0 || y > 0, AmmError::ZeroBalance);

        if x > 0 {
            self.send_from_treasury(true, x)?;
        }
        if y > 0 {
            self.send_from_treasury(false, y)?;
        }
        Ok(())
    }

    pub fn send_from_treasury(&self, is_x: bool, amount: u64) -> Result<()> {
        // YOUR TURN: model this on withdraw_tokens in withdraw.rs.
        // 1. pick (from, to): treasury_x -> authority_x, or treasury_y -> authority_y
        // 2. build signer_seeds for the config PDA
        // 3. transfer(CpiContext::new_with_signer(...), amount)
        todo!()
    }
}
