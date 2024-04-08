use anchor_lang::prelude::*;

use instructions::*;
use state::*;

mod errors;
mod instructions;
mod state;

declare_id!("A8Tx8hQwFY2mDuRG89dxiLBac1V3rJtkpPKvktc7uMkw");

#[program]
pub mod devolt {

    use super::*;

    pub fn retrieve_station(ctx: Context<RetrieveStation>) -> Result<()> {
        instructions::retrieve_station(ctx)
    }

    pub fn battery_report(ctx: Context<CreateBateryReport>, args: CreateBateryReportArgs) -> Result<()> {
        instructions::battery_report(ctx, args)
    }

    pub fn place_bid(ctx: Context<PlaceBid>, args: PlaceBidArgs) -> Result<()> {
        instructions::place_bid(ctx, args)
    }
}
