use anchor_lang::prelude::*;

use crate::{
    state::{CreateBateryReportArgs, Station},
    Auction, Bid,
};

#[derive(Debug, Accounts)]
#[instruction(args: CreateBateryReportArgs)]
pub struct CreateBateryReport<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(init_if_needed, payer = signer, space = Station::SPACE, seeds = [Station::PREFIX_SEED.as_ref(), args.id.as_ref()], bump)]
    pub station: Account<'info, Station>,

    pub system_program: Program<'info, System>,
}

pub fn battery_report(
    ctx: Context<CreateBateryReport>,
    args: CreateBateryReportArgs,
) -> Result<()> {
    let station: &mut Account<Station> = &mut ctx.accounts.station;

    msg!("\nINFO: Instruction: Battery Report");

    station.auth = *ctx.accounts.signer.key;
    station.id = args.id;
    station.latitude = args.latitude;
    station.longitude = args.longitude;
    station.max_capacity = args.max_capacity;
    station.battery_level = args.battery_level;

    let clock = Clock::get().unwrap();
    let _current_timestamp = clock.unix_timestamp as u64;

    if let Some(auction) = &mut station.auction {
        if auction.ongoing {
            let clock = Clock::get().unwrap();
            let current_timestamp = clock.unix_timestamp as u64;
            if auction.timestamp <= current_timestamp {
                msg!(
                    "\nINFO: Auction finalized for station with ID: {}",
                    station.id
                );
                finalize_auction(station, current_timestamp);
            }
        }
    } else {
        let battery_deficit_percentage = (station.max_capacity - station.battery_level) / station.max_capacity * 100.0;
        if battery_deficit_percentage >= 20.0 {
            if let Some(auction) = &mut station.auction {
                if !auction.ongoing {
                    msg!("Creating auction for {}", station.id);
                    create_auction(station, battery_deficit_percentage, _current_timestamp);
                }
            } else {
                msg!("Creating auction for {}", station.id);
                create_auction(station, battery_deficit_percentage, _current_timestamp);
            }
        }
    }

    msg!("\nINFO: Station: {:?}", station);

    Ok(())
}

pub fn finalize_auction(station: &mut Account<Station>, timestamp: u64) {
    let station_id = station.id.clone();
    msg!("Finalizing auction for station with ID: {}", station_id);

    let total_amount: f64;

    if let Some(auction) = &mut station.auction {
        if auction.ongoing && auction.timestamp + auction.duration <= timestamp {
            let mut accepted_bids: Vec<Bid> = Vec::new();
            let mut energy_needed = auction.req_charge;

            auction.bids.sort_by(|a, b| {
                a.amount
                    .partial_cmp(&b.amount)
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            for bid in &auction.bids {
                if energy_needed <= 0.0 {
                    break;
                }

                let bid_to_accept = if bid.amount <= energy_needed {
                    bid.clone()
                } else {
                    Bid {
                        bidder: bid.bidder.clone(),
                        amount: energy_needed,
                        price_per_amount: bid.price_per_amount,
                    }
                };

                accepted_bids.push(bid_to_accept.clone());
                energy_needed -= bid_to_accept.amount;
            }

            total_amount = accepted_bids.iter().map(|bid| bid.amount).sum();

            // Log out each winning bid
            for accepted_bid in accepted_bids {
                msg!("Winning bid: Bidder: {}, Amount: {}", accepted_bid.bidder, accepted_bid.amount);
                auction.winning_bids.push(accepted_bid);
            }

            auction.ongoing = false;
        } else {
            total_amount = 0.0;
        }
    } else {
        total_amount = 0.0;
    }

    if total_amount > 0.0 {
        station.battery_level += total_amount;
        msg!(
            "Auction finalized and battery level updated for station with ID: {}",
            station_id
        );
    } else {
        msg!(
            "No auction to finalize or no bids accepted for station with ID: {}",
            station_id
        );
    }
}

pub fn create_auction(station: &mut Account<Station>, req_charge: f64, timestamp: u64) {
    msg!("Creating auction for station with ID: {}", station.id);

    let new_auction = Auction {
        req_charge,
        timestamp,
        duration: 60,
        bids: Vec::new(),
        ongoing: true,
        winning_bids: Vec::new(),
    };

    msg!("Auction created for station with ID: {}", station.id);
    station.auction = Some(new_auction);
}
