use super::{PositionOperationType};

#[derive(Debug, Clone)]
pub struct OrderData{
    pub id : String,
    pub wallet_id : String,
    pub asset_pair_id : String,
    pub invest_amount : f64,
    pub leverage : String,
    pub create_date : u64,
    pub position_operation : PositionOperationType,
    pub take_profit_in_currency : Option<f64>,
    pub stop_loss_in_currency : Option<f64>,
    pub take_profit_rate : Option<f64>,
    pub stop_loss_rate : Option<f64>,
    pub last_update_date : u64,
    pub process_id : String,
    pub volume : f64,
    pub profit: f64,
    pub stop_out_percent: f64
}