use prost::DecodeError;

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BidAsk {
    #[prost(string, tag = "1")]
    pub id: String,
    #[prost(fixed64, tag = "2")]
    pub datetime: u64,
    #[prost(double, tag = "3")]
    pub bid: f64,
    #[prost(double, tag = "4")]
    pub ask: f64,
}

impl BidAsk {
    pub fn parse(payload: &[u8]) -> Result<Self, DecodeError> {
        prost::Message::decode(payload)
    }
}

#[derive(Clone, PartialEq, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum PositionOperationType {
    Buy = 0,
    Sell = 1,
}

impl PositionOperationType {
    pub fn get_close_price(&self, bid_ask: &BidAsk) -> f64 {
        match self {
            PositionOperationType::Buy => bid_ask.bid,
            PositionOperationType::Sell => bid_ask.ask,
        }
    }

    pub fn get_open_price(&self, bid_ask: &BidAsk) -> f64 {
        match self {
            PositionOperationType::Buy => bid_ask.ask,
            PositionOperationType::Sell => bid_ask.bid,
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub enum ClosePositionReason {
//     None,
//     ClientCommand,
//     StopOut,
//     TakeProfit,
//     StopLoss,
//     Canceled,
//     AdminAction,
// }

#[derive(Clone, PartialEq, Debug, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClosePositionReason {
    None = 0,
    ClientCommand = 1,
    StopOut = 2,
    TakeProfit = 3,
    StopLoss = 4,
    Canceled = 5,
    AdminAction = 6,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PositionOrderType {
    Market,
    BuyLimit,
    BuyStop,
    SellLimit,
    SellStop,
}

#[derive(Debug, Clone)]
pub struct PositionSwap {
    pub datetime: u128,
    pub executedDt: u128,
    pub quoteDt: Option<u128>,
    pub long: f64,
    pub short: f64,
    pub amount: u32,
    pub profit: f64,
}
