use anchor_lang::prelude::*;

use crate::{
    state::Station, Bid, PlaceBidArgs
};

#[derive(Debug, Accounts)]
#[instruction(args: PlaceBidArgs)]
pub struct PlaceBid<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // #[account(init, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    #[account(init_if_needed, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    pub station: Account<'info, Station>,

    pub system_program: Program<'info, System>,
}

pub fn place_bid(ctx: Context<PlaceBid>, args: PlaceBidArgs) -> Result<()> {
    let station: &mut Account<Station> = &mut ctx.accounts.station;

    msg!("\nINFO: Instruction: Place Bid");

    let station_id = args.id.clone();
    let bidder = args.bidder.clone();
    let amount = args.amount.clone();
    let price_per_amount = args.price_per_amount.clone();

    if let Some(auction) = &mut station.auction {
        if auction.ongoing {
            let clock = Clock::get().unwrap();
            let current_timestamp = clock.unix_timestamp as u64;
            if auction.timestamp <= current_timestamp {
                msg!(
                    "\nINFO: Placing bid for auction in station with ID: {}",
                    station_id
                );
                let bid = Bid {
                    bidder,
                    amount,
                    price_per_amount,
                };
                msg!("Bid: {:?}", bid);
                auction.bids.push(bid);
            } else {
                msg!(
                    "\nERROR: Auction for station with ID: {} has already ended.",
                    station_id
                );
            }
        } else {
            msg!(
                "\nERROR: No ongoing auction for station with ID: {}",
                station_id
            );
        }
    } else {
        msg!(
            "\nERROR: No auction found for station with ID: {}",
            station_id
        );
    }

    msg!("Station: {:?}", station);
    
    // station.exit(&ctx.program_id)?;

    Ok(())
}
