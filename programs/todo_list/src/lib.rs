use anchor_lang::prelude::*;

declare_id!("ArjFD4yXdGkcKaSeYEtvya9BcyoncH2TcPhSeJLxKAz1");

#[program]
pub mod todo_list {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
