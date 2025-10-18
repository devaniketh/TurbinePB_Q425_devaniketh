use anchor_lang::prelude::*;

declare_id!("bN8nVVScuWXwpBG8DQBhiUoAMMC5eNmfK6WxLKmQq4m");

#[program]
pub mod hello_anchor {
    use super::*;

    pub fn say_hello(_ctx: Context<SayHello>) -> Result<()> {
        msg!("Hello, Anchor World!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SayHello {}
