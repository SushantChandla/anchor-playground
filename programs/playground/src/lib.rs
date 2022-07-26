use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod playground {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        println!("Program Log: {:?}, {:?}, {:?}",ctx.accounts.global_state.key(),ctx.accounts.owner,ctx.accounts.global_state.key() );
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        mut,
    )]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = ["seed".as_bytes()],
        bump,
        space = 200+8,
    )]
    pub global_state: Box<Account<'info, GlobalState>>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct GlobalState {
    pub counter: u32,
}