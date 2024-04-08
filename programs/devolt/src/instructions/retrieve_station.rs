use anchor_lang::prelude::*;

use crate::state::Station;

#[derive(Debug, Accounts)]
pub struct RetrieveStation<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init_if_needed, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref()], bump)]
    pub station: Account<'info, Station>,

    pub system_program: Program<'info, System>,
}

pub fn retrieve_station(ctx: Context<RetrieveStation>) -> Result<()> {
    let station = &mut ctx.accounts.station;

    msg!("\nINFO: Instruction: Retrieve Station");
    msg!("\nINFO: Station: {:?}", station);

    Ok(())
}