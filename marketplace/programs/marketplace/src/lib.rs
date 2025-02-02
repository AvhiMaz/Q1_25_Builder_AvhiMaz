use anchor_lang::prelude::*;

declare_id!("9ZKMs25fhd4ihPU7PT2ymTzC4Z99GG6gnnN17iNKQHue");

mod state;
mod contexts;

mod error;

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
