use anchor_lang::prelude::*;

#[derive(Debug)]
#[account]
pub struct Station {
    pub auth: Pubkey,
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub max_capacity: f64,
    pub battery_level: f64,
    pub auction: Option<Auction>,
}

impl Station {
    #[allow(elided_lifetimes_in_associated_constant)]
    /// static prefix seed string used to derive the PDAs
    pub const PREFIX_SEED: &[u8] = b"station";

    /// total on-chain space needed to allocate the account
    pub const SPACE: usize =
        // anchor descriminator + all static variables
        8 + std::mem::size_of::<Self>();
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Bid {
    pub bidder: String,
    pub amount: f64,
    pub price_per_amount: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct Auction {
    pub req_charge: f64,
    pub timestamp: u64,
    pub duration: u64,
    pub bids: Vec<Bid>,
    pub ongoing: bool,
    pub winning_bids: Vec<Bid>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct CreateBateryReportArgs {
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub max_capacity: f64,
    pub battery_level: f64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct PlaceBidArgs {
    pub id: String,
    pub bidder: String,
    pub amount: f64,
    pub price_per_amount: f64,
}

