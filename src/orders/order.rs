use crate::{get_current_date, service_bus::ClosedPositionSbModel};

use super::{
    active_order_state::ActiveOrderStateInfo, closed_order_state::ClosedOrderStateInfo, BidAsk,
    ClosePositionReason, OrderData, PendingOrderStateInfo,
};

pub enum Order {
    Active(ActiveOrderStateInfo, OrderData),
    Closed(ClosedOrderStateInfo, OrderData),
    Pending(PendingOrderStateInfo, OrderData),
}

impl Order {
    pub fn get_open_date(&self) -> u64 {
        match self {
            Order::Active(_, data) => data.create_date,
            Order::Closed(_, data) => data.create_date,
            Order::Pending(_, data) => data.create_date,
        }
    }

    pub fn get_wallet(&self) -> String {
        match self {
            Order::Active(_, data) => data.wallet_id.clone(),
            Order::Closed(_, data) => data.wallet_id.clone(),
            Order::Pending(_, data) => data.wallet_id.clone(),
        }
    }

    pub fn get_id(&self) -> String {
        match self {
            Order::Active(_, data) => data.id.clone(),
            Order::Closed(_, data) => data.id.clone(),
            Order::Pending(_, data) => data.id.clone(),
        }
    }

    pub fn to_sb_closed_order(self) -> ClosedPositionSbModel {
        match self {
            Order::Active(_, _) => panic!("Active order cant be converted to closed sb model"),
            Order::Closed(state, data) => {
                return ClosedPositionSbModel {
                    id: data.id,
                    wallet_id: data.wallet_id,
                    asset_pair_id: data.asset_pair_id,
                    invest_amount: data.invest_amount,
                    leverage: data.leverage,
                    create_date: data.create_date,
                    position_operation: data.position_operation as i32,
                    take_profit_in_currency: data.take_profit_in_currency,
                    stop_loss_in_currency: data.stop_loss_in_currency,
                    take_profit_rate: data.take_profit_rate,
                    stop_loss_rate: data.stop_loss_rate,
                    last_update_date: data.last_update_date,
                    process_id: data.process_id,
                    volume: data.volume,
                    profit: data.profit,
                    stop_out_percent: data.stop_out_percent,
                    //TODO - add close price
                    close_price: 0.0,
                    close_bid_ask: Some(state.close_bid_ask),
                    close_date: state.close_date,
                    burn_bonus: 0.0,
                    close_reason: state.close_reason as i32,
                    open_price: state.active_order_state.open_price,
                    open_bid_ask: Some(state.active_order_state.open_bid_ask),
                    open_date: state.active_order_state.open_date,
                };
            }
            Order::Pending(_, _) => panic!("Pending order cant be converted to closed sb model"),
        }
    }

    pub fn convert_to_close_position(
        self,
        close_reason: ClosePositionReason,
        close_bid_ask: &BidAsk,
    ) -> Self {
        match self {
            Order::Active(state, order_data) => {
                let close_state = ClosedOrderStateInfo::from_active(
                    state,
                    close_bid_ask.clone(),
                    order_data.position_operation.get_close_price(close_bid_ask),
                    get_current_date(),
                    0.0,
                    close_reason,
                );

                return Self::Closed(close_state, order_data);
            }
            Order::Closed(_, _) => panic!("position alredy closed. cant close"),
            Order::Pending(_, _) => panic!("order pending. cant convert into closed"),
        }
    }

    pub fn is_stop_out_triggered(&self) -> bool {
        match self {
            Order::Active(active_order_info, order_data) => {
                let position_margin = order_data.profit + order_data.invest_amount;
                let position_margin_percent = position_margin / order_data.invest_amount * 100.0;

                return 100.0 - position_margin_percent >= order_data.stop_out_percent;
            }
            Order::Closed(_, _) => panic!("order alredy closed. Stop out cant be triggered"),
            Order::Pending(_, _) => panic!("order pending. Stop out cant be triggered"),
        }
    }

    pub fn is_take_profit_triggered(&self, bid_ask: &BidAsk) -> bool {
        match self {
            Order::Active(active_order_info, order_data) => {
                if order_data.take_profit_in_currency.is_some() {
                    return order_data.profit >= order_data.take_profit_in_currency.unwrap();
                }

                if order_data.take_profit_rate.is_some() {
                    let result = match order_data.position_operation {
                        super::PositionOperationType::Buy => {
                            order_data.take_profit_rate.unwrap() <= bid_ask.bid
                        }
                        super::PositionOperationType::Sell => {
                            order_data.take_profit_rate.unwrap() >= bid_ask.ask
                        }
                    };
                    return result;
                }

                return false;
            }
            Order::Closed(_, _) => panic!("order alredy closed. take profit cant be triggered"),
            Order::Pending(_, _) => panic!("order pending. take profit cant be triggered"),
        }
    }

    pub fn update_profit_rate(&mut self, bid_ask: &BidAsk) {
        match self {
            Order::Active(_, order_data) => {
                let open_price = order_data.position_operation.get_open_price(bid_ask);
                let close_price = order_data.position_operation.get_close_price(bid_ask);

                match order_data.position_operation {
                    super::PositionOperationType::Buy => {
                        order_data.profit = (close_price / open_price - 1.0) * order_data.volume
                    }
                    super::PositionOperationType::Sell => {
                        order_data.profit = (close_price / open_price - 1.0) * -order_data.volume
                    }
                }
            }
            Order::Closed(_, _) => panic!("cant calculate profit for closed position"),
            Order::Pending(_, _) => panic!("cant calculate profit for pending order"),
        }
    }

    pub fn is_stop_loss_triggered(&self, bid_ask: &BidAsk) -> bool {
        match self {
            Order::Active(active_order_info, order_data) => {
                if order_data.stop_loss_in_currency.is_some() {
                    return order_data.profit <= order_data.take_profit_in_currency.unwrap();
                }

                if order_data.stop_loss_rate.is_some() {
                    let result = match order_data.position_operation {
                        super::PositionOperationType::Buy => {
                            order_data.stop_loss_rate.unwrap() >= bid_ask.bid
                        }
                        super::PositionOperationType::Sell => {
                            order_data.stop_loss_rate.unwrap() <= bid_ask.ask
                        }
                    };
                    return result;
                }

                return false;
            }
            Order::Closed(_, _) => panic!("order alredy closed. stop loss cant be triggered"),
            Order::Pending(_, _) => panic!("order pending. stop loss cant be triggered"),
        }
    }

    pub fn convert_pending_to_active(
        self,
        open_price: f64,
        open_bid_ask: BidAsk,
        topping_up_percent: u8,
    ) -> Self {
        match self {
            Order::Active(_, _) => panic!("cant execute pending for active"),
            Order::Closed(_, _) => panic!("cant execute active order"),
            Order::Pending(pending_state, order_data) => {
                let active_state = ActiveOrderStateInfo::from_pending_order(
                    pending_state,
                    open_price,
                    open_bid_ask,
                    topping_up_percent,
                );
                return Self::Active(active_state, order_data);
            }
        }
    }

    pub fn get_order_data(&self) -> &OrderData {
        match self {
            Order::Active(_, order_data) => order_data,
            Order::Closed(_, order_data) => order_data,
            Order::Pending(_, order_data) => order_data,
        }
    }

    pub fn create_active(
        order_data: OrderData,
        open_price: f64,
        open_bid_ask: BidAsk,
        topping_up_percent: u8,
    ) -> Self {
        let active_state = ActiveOrderStateInfo::new(open_price, open_bid_ask, topping_up_percent);
        return Self::Active(active_state, order_data);
    }

    pub fn create_pending(order_data: OrderData, desire_price: f64) -> Self {
        let pending_state = PendingOrderStateInfo::new(desire_price);
        return Self::Pending(pending_state, order_data);
    }
}
