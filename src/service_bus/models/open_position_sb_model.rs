use prost::DecodeError;

use crate::orders::{ActiveOrderStateInfo, BidAsk, Order, OrderData, PositionOperationType};

#[derive(PartialEq, ::prost::Message)]
pub struct OpenPositionCommand {
    #[prost(string, tag = "1")]
    pub wallet_id: String,
    #[prost(string, tag = "2")]
    pub asset_pair_id: String,
    #[prost(double, tag = "3")]
    pub invest_amount: f64,
    #[prost(string, tag = "4")]
    pub leverage: String,
    #[prost(enumeration = "PositionOperationType", tag = "5")]
    pub position_operation: i32,
    #[prost(message, tag = "6")]
    pub take_profit_in_currency: Option<f64>,
    #[prost(message, tag = "7")]
    pub stop_loss_in_currency: Option<f64>,
    #[prost(message, tag = "8")]
    pub take_profit_rate: Option<f64>,
    #[prost(message, tag = "9")]
    pub stop_loss_rate: Option<f64>,
    #[prost(uint64, tag = "10")]
    pub last_update_date: u64,
    #[prost(string, tag = "11")]
    pub process_id: String,
    #[prost(double, tag = "12")]
    pub volume: f64,
    #[prost(double, tag = "13")]
    pub profit: f64,
    #[prost(double, tag = "14")]
    pub stop_out_percent: f64,
    #[prost(string, tag = "15")]
    pub request_id: String,
}

#[derive(Clone, PartialEq, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum OpenPositionResult {
    Ok = 1,
    NoLiquidity = 2,
}

#[derive(PartialEq, ::prost::Message)]
pub struct OpenPositonSbMessage {
    #[prost(string, tag = "1")]
    pub wallet_id: String,
    #[prost(message, tag = "2")]
    pub id: Option<String>,
    #[prost(string, tag = "3")]
    pub request_id: String,
    #[prost(message, tag = "4")]
    pub open_date: Option<u64>,
    #[prost(enumeration = "OpenPositionResult", tag = "5")]
    pub status: i32,
}

impl OpenPositonSbMessage {
    pub fn new(
        position_operation: OpenPositionResult,
        id: Option<String>,
        request_id: String,
        wallet_id: String,
        open_date: Option<u64>,
    ) -> OpenPositonSbMessage {
        OpenPositonSbMessage {
            wallet_id,
            id,
            request_id,
            open_date,
            status: position_operation as i32
        }
    }
}

impl OpenPositionCommand {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }

    pub fn get_order_type(&self) -> PositionOperationType {
        match PositionOperationType::from_i32(self.position_operation) {
            Some(int) => int,
            None => panic!("Invalid position operation type"),
        }
    }

    pub fn to_active_order(self, open_bidask: BidAsk, open_price: f64) -> Order {
        let order_data = OrderData {
            id: generate_id(),
            asset_pair_id: self.asset_pair_id,
            invest_amount: self.invest_amount,
            leverage: self.leverage,
            position_operation: PositionOperationType::from_i32(self.position_operation).unwrap(),
            take_profit_in_currency: self.take_profit_in_currency,
            stop_loss_in_currency: self.stop_loss_in_currency,
            wallet_id: self.wallet_id,
            create_date: get_current_date(),
            take_profit_rate: self.take_profit_rate,
            stop_loss_rate: self.stop_loss_rate,
            last_update_date: get_current_date(),
            process_id: self.process_id,
            volume: self.volume,
            profit: 0.0,
            stop_out_percent: self.stop_out_percent,
        };

        let active_order_state = ActiveOrderStateInfo {
            open_price: open_price,
            open_bid_ask: open_bidask,
            open_date: get_current_date(),
            pending_order_state: None,
        };

        Order::Active(active_order_state, order_data)
    }
}
fn generate_id() -> String {
    "asdq".into()
}

fn get_current_date() -> u64 {
    return 0;
}

fn get_open_price() -> f64 {
    return 0.0;
}
