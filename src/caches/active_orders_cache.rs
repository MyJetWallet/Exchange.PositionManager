use std::collections::{BTreeMap, HashMap};

use tokio::sync::RwLock;

use crate::orders::{BidAsk, ClosePositionReason, Order};

pub struct ActiveOrdersCache {
    active_orders_cache: RwLock<BTreeMap<String, HashMap<String, Order>>>,
}

impl ActiveOrdersCache {
    pub fn new() -> ActiveOrdersCache{
        ActiveOrdersCache{
            active_orders_cache: RwLock::new(BTreeMap::new())
        }
    }

    pub async fn update_rate_and_close_positions(&self, bid_ask: &BidAsk) -> Option<Vec<Order>> {
        let mut orders_to_close: HashMap<&String, ClosePositionReason> = HashMap::new();
        {
            let mut write = self.active_orders_cache.write().await;

            let asset_orders = write.get_mut(&bid_ask.id);

            let removed_orders: Option<String> = match asset_orders {
                
                Some(orders_to_check) => {
                    for (id, order) in orders_to_check {
                        order.update_profit_rate(bid_ask);

                        if order.is_stop_loss_triggered(bid_ask) {
                            orders_to_close.insert(id, ClosePositionReason::StopLoss);
                            continue;
                        }

                        if order.is_stop_out_triggered() {
                            orders_to_close.insert(id, ClosePositionReason::StopOut);
                            continue;
                        }

                        if order.is_take_profit_triggered(bid_ask) {
                            orders_to_close.insert(id, ClosePositionReason::TakeProfit);
                        }
                    }

                    return Some(Vec::new());
                }
                None => {
                    return None;
                }
            };

            if removed_orders.is_none() {
                return None;
            }
        }

        let mut write = self.active_orders_cache.write().await;
        let instrument_to_remove = write.get_mut(&bid_ask.id);

        let removed_orders_from_cache: Vec<Order> = Vec::new();

        for (id, close_reason) in orders_to_close {
            let removed_order = instrument_to_remove.unwrap().remove(id);

            match removed_order {
                Some(order) => {
                    let closed_order = order.convert_to_close_position(close_reason, bid_ask);
                    removed_orders_from_cache.push(closed_order);
                }
                None => {}
            }
        }

        return Some(removed_orders_from_cache);
    }

    pub async fn set_active_order(&self, order: Order) {
        match &order {
            Order::Active(_, order_data) => {
                let mut write_access = self.active_orders_cache.write().await;
                let account_orders = write_access.get_mut(&order_data.asset_pair_id);

                match account_orders {
                    Some(orders) => {
                        orders.insert(order_data.id.clone(), order);
                    }
                    None => {
                        let mut hm = HashMap::new();
                        let asset_id = order_data.asset_pair_id.clone();
                        hm.insert(order_data.id.clone(), order);
                        write_access.insert(asset_id, hm);
                    }
                }
            }
            Order::Closed(_, _) => panic!("Cant set closed order as active"),
            Order::Pending(_, _) => panic!("Cant set pending order as active"),
        }
    }

    pub async fn remove_order(&self, asset_id: &String, order_id: &String) -> Option<Order> {
        let mut write_access = self.active_orders_cache.write().await;
        let asset_orders = write_access.get_mut(asset_id);

        match asset_orders {
            Some(orders) => {
                orders.remove(order_id)
            },
            None => None,
        }
    }
}
