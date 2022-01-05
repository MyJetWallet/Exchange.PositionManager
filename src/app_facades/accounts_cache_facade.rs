use async_trait::async_trait;

use crate::orders::{Order, BidAsk};

#[async_trait]
pub trait AccountsCacheFacade{
    async fn set_active_order(&self, order: Order);
    async fn update_rate_and_close_positions(&self, bid_ask: &BidAsk) -> Option<Vec<Order>>;
    async fn remove_order(&self, asset_id: &String, order_id: &String) -> Option<Order>;
}