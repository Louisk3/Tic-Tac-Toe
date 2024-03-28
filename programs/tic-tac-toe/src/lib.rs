use anchor_lang::prelude::*;

declare_id!("DvxtSzdJj1expcrVZVS9cC6cgs4SbewyJZCVbCeMFDpE");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
