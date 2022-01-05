use crate::orders::{BidAsk, ClosePositionReason, PositionOperationType};

#[derive(PartialEq, ::prost::Message)]
pub struct ClosePositionSbCommand {
    #[prost(string, tag = "1")]
    pub position_id: String,
    #[prost(string, tag = "2")]
    pub asset_pair_id: String,
}

#[derive(PartialEq, ::prost::Message)]
pub struct ClosedPositionSbModel {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(string, tag = "2")]
    pub wallet_id: String,
    #[prost(string, tag = "3")]
    pub asset_pair_id: String,
    #[prost(double, tag = "4")]
    pub invest_amount: f64,
    #[prost(string, tag = "5")]
    pub leverage: String,
    #[prost(uint64, tag = "6")]
    pub create_date: u64,
    #[prost(enumeration = "PositionOperationType", tag = "7")]
    pub position_operation: i32,
    #[prost(message, tag = "8")]
    pub take_profit_in_currency: Option<f64>,
    #[prost(message, tag = "9")]
    pub stop_loss_in_currency: Option<f64>,
    #[prost(message, tag = "10")]
    pub take_profit_rate: Option<f64>,
    #[prost(message, tag = "11")]
    pub stop_loss_rate: Option<f64>,
    #[prost(uint64, tag = "12")]
    pub last_update_date: u64,
    #[prost(string, tag = "13")]
    pub process_id: String,
    #[prost(double, tag = "14")]
    pub volume: f64,
    #[prost(double, tag = "15")]
    pub profit: f64,
    #[prost(double, tag = "16")]
    pub stop_out_percent: f64,
    #[prost(message, tag = "17")]
    pub close_bid_ask: Option<BidAsk>,
    #[prost(double, tag = "18")]
    pub close_price: f64,
    #[prost(uint64, tag = "19")]
    pub close_date: u64,
    #[prost(double, tag = "20")]
    pub burn_bonus: f64,
    #[prost(enumeration = "ClosePositionReason", tag = "21")]
    pub close_reason: i32,
    #[prost(double, tag = "22")]
    pub open_price: f64,
    #[prost(message, tag = "23")]
    pub open_bid_ask: Option<BidAsk>,
    #[prost(uint64, tag = "24")]
    pub open_date: u64,
}
