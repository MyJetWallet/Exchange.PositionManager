use super::{active_order_state::ActiveOrderStateInfo, BidAsk, ClosePositionReason};

pub struct ClosedOrderStateInfo {
    pub close_bid_ask: BidAsk,
    pub close_price: f64,
    pub close_date: u64,
    pub burn_bonus: f64,
    pub close_reason: ClosePositionReason,
    pub active_order_state: ActiveOrderStateInfo,
}

impl ClosedOrderStateInfo {
    pub fn from_active(
        active_order_state: ActiveOrderStateInfo,
        close_bid_ask: BidAsk,
        close_price: f64,
        close_date: u64,
        burn_bonus: f64,
        close_reason: ClosePositionReason,
    ) -> ClosedOrderStateInfo {
        ClosedOrderStateInfo {
            close_bid_ask,
            close_price,
            close_date,
            burn_bonus,
            close_reason,
            active_order_state,
        }
    }
}
