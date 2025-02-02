use anchor_lang::prelude::*;
use anchor_spl::{ token::Token, token_interface::Mint };

use crate::{ error::MarketPlaceError, state::MarketPlace };

#[derive(Accounts)]
#[instruction(name : String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
        space = MarketPlace::INIT_SPACE
    )]
    pub marketplace: Account<'info, MarketPlace>,

    #[account(seeds = [b"treasury", marketplace.key().as_ref()], bump)]
    pub treasury: SystemAccount<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"reward", marketplace.key().as_ref()],
        bump,
        mint::authority = marketplace,
        mint::decimals = 6
    )]
    pub reward_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, name: String, bump: InitializeBumps) -> Result<()> {
        require!(name.len() > 0 && name.len() < 4 + 33, MarketPlaceError::NameTooLong);
        self.marketplace.set_inner(MarketPlace {
            admin: self.admin.key(),
            fee: 1,
            bump: bump.marketplace,
            treasury_bump: bump.treasury,
            reward_mint_bump: bump.reward_mint,
            name,
        });

        Ok(())
    }
}
