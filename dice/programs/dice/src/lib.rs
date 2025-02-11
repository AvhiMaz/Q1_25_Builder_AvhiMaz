use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod contexts;
pub mod state;

pub use constants::*;
pub use contexts::*;
pub use state::*;

declare_id!("F7h7MxSPuyfJqCFnCp6NYsQooTtqCPVALshQp6qK899k");

#[program]
pub mod dice {
    use super::*;

}
