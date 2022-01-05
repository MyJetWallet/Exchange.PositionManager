use super::{BidAsk, PendingOrderStateInfo, PositionSwap};

pub struct ActiveOrderStateInfo{
    pub open_price: f64,
    pub open_bid_ask: BidAsk,
    pub open_date: u64,
    // pub swaps: Vec<PositionSwap>,
    // pub topping_up_percent: u8,
    // pub reserved_fund_for_topping_up: f64,
    pub pending_order_state: Option<PendingOrderStateInfo>
}

impl ActiveOrderStateInfo {
    pub fn from_pending_order(pending_state: PendingOrderStateInfo, open_price: f64, open_bid_ask: BidAsk, topping_up_percent: u8) -> ActiveOrderStateInfo{        
        let pending = pending_state.clone();

        ActiveOrderStateInfo{
            open_price,
            open_bid_ask,
            open_date: 0,
            // swaps: Vec::new(),
            // topping_up_percent,
            // reserved_fund_for_topping_up: 0.0,
            pending_order_state: Some(pending),
        }
    }

    pub fn new(open_price: f64, open_bid_ask: BidAsk, topping_up_percent: u8) -> ActiveOrderStateInfo{        
        ActiveOrderStateInfo{
            open_price,
            open_bid_ask,
            open_date: 0,
            pending_order_state: None,
        }
    }
}